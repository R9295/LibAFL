use std::{borrow::Cow, path::PathBuf, time::Duration};

use libafl::{
    corpus::{Corpus, OnDiskCorpus},
    events::{CentralizedEventManager, EventManagerHooksTuple, LlmpRestartingEventManager},
    executors::forkserver::{ForkserverExecutor, ForkserverExecutorBuilder},
    feedback_and, feedback_or, feedback_or_fast,
    feedbacks::{ConstFeedback, CrashFeedback, MaxMapFeedback, TimeFeedback, TimeoutFeedback},
    fuzzer::{Fuzzer, StdFuzzer},
    inputs::BytesInput,
    mutators::{
        scheduled::havoc_mutations, tokens_mutations, AFLppRedQueen, StdScheduledMutator, Tokens,
    },
    observers::{CanTrack, HitcountsMapObserver, StdMapObserver, TimeObserver},
    schedulers::{
        powersched::PowerSchedule, IndexesLenTimeMinimizerScheduler, StdWeightedScheduler,
    },
    stages::{
        mutational::MultiMutationalStage, CalibrationStage, ColorizationStage, IfStage,
        StdPowerMutationalStage,
    },
    state::{HasCorpus, HasCurrentTestcase, HasStartTime, StdState},
    Error, HasMetadata, HasFeedback,
};
use libafl_bolts::{
    current_nanos, current_time,
    fs::get_unique_std_input_file,
    ownedref::OwnedRefMut,
    rands::StdRand,
    shmem::{ShMem, ShMemProvider, StdShMemProvider},
    tuples::{tuple_list, Handled, Merge},
    AsSliceMut,
};
use libafl_targets::{cmps::AFLppCmpLogMap, AFLppCmpLogObserver, AFLppCmplogTracingStage};
use serde::{Deserialize, Serialize};

use crate::{
    afl_stats::AflStatsStage,
    corpus::{set_corpus_filepath, set_solution_filepath},
    feedback::{filepath::CustomFilepathToTestcaseFeedback, seed::SeedFeedback},
    run_fuzzer_with_stage, Opt, AFL_DEFAULT_INPUT_LEN_MAX, AFL_DEFAULT_INPUT_LEN_MIN,
    SHMEM_ENV_VAR,
};

#[allow(clippy::too_many_lines)]
pub fn run_client<EMH, SP>(
    state: Option<LibaflFuzzState>,
    mut restarting_mgr: CentralizedEventManager<
        LlmpRestartingEventManager<(), LibaflFuzzState, SP>,
        EMH,
        LibaflFuzzState,
        SP,
    >,
    fuzzer_dir: &PathBuf,
    opt: &Opt,
) -> Result<(), Error>
where
    EMH: EventManagerHooksTuple<LibaflFuzzState> + Copy + Clone,
    SP: ShMemProvider,
{
    // Create the shared memory map for comms with the forkserver
    let mut shmem_provider = StdShMemProvider::new().unwrap();
    let mut shmem = shmem_provider.new_shmem(opt.map_size).unwrap();
    shmem.write_to_env(SHMEM_ENV_VAR).unwrap();
    let shmem_buf = shmem.as_slice_mut();

    // Create an observation channel to keep track of edges hit.
    let edges_observer = unsafe {
        HitcountsMapObserver::new(StdMapObserver::new("edges", shmem_buf)).track_indices()
    };

    // Create a MapFeedback for coverage guided fuzzin'
    let map_feedback = MaxMapFeedback::new(&edges_observer);

    // Create the CalibrationStage; used to measure the stability of an input.
    let calibration = CalibrationStage::new(&map_feedback);

    // Create an observation channel to keep track of the execution time.
    let time_observer = TimeObserver::new("time");

    /*
     * Feedback to decide if the Input is "corpus worthy"
     * We only check if it gives new coverage.
     * The `TimeFeedback` is used to annotate the testcase with it's exec time.
     * The `CustomFilepathToTestcaseFeedback is used to adhere to AFL++'s corpus format.
     * The `Seedfeedback` is used during seed loading to adhere to AFL++'s handling of seeds
     */
    let mut feedback = SeedFeedback::new(
        feedback_or!(
            feedback_or!(map_feedback, TimeFeedback::new(&time_observer)),
            CustomFilepathToTestcaseFeedback::new(set_corpus_filepath, fuzzer_dir.clone())
        ),
        opt,
    );

    /*
     * Feedback to decide if the Input is "solution worthy".
     * We check if it's a crash or a timeout (if we are configured to consider timeouts)
     * The `CustomFilepathToTestcaseFeedback is used to adhere to AFL++'s corpus format.
     * The `MaxMapFeedback` saves objectives only if they hit new edges
     * */
    let mut objective = feedback_or!(
        feedback_and!(
            feedback_or_fast!(
                CrashFeedback::new(),
                feedback_and!(
                    ConstFeedback::new(!opt.ignore_timeouts),
                    TimeoutFeedback::new()
                )
            ),
            MaxMapFeedback::with_name("edges_objective", &edges_observer)
        ),
        CustomFilepathToTestcaseFeedback::new(set_solution_filepath, fuzzer_dir.clone())
    );

    // Initialize our State if necessary
    let mut state = state.unwrap_or_else(|| {
        StdState::new(
            StdRand::with_seed(current_nanos()),
            OnDiskCorpus::<BytesInput>::new(fuzzer_dir.join("queue")).unwrap(),
            OnDiskCorpus::<BytesInput>::new(fuzzer_dir.clone()).unwrap(),
            &mut feedback,
            &mut objective,
        )
        .unwrap()
    });

    // Create our Mutational Stage.
    let power = StdPowerMutationalStage::new(StdScheduledMutator::new(
        havoc_mutations().merge(tokens_mutations()),
    ));
    let strategy = opt.power_schedule.unwrap_or(PowerSchedule::EXPLORE);

    // Create our ColorizationStage
    let colorization = ColorizationStage::new(&edges_observer);

    // Create our Scheduler
    let mut weighted_scheduler =
        StdWeightedScheduler::with_schedule(&mut state, &edges_observer, Some(strategy.into()));
    if opt.cycle_schedules {
        weighted_scheduler = weighted_scheduler.cycling_scheduler();
    }
    let scheduler = IndexesLenTimeMinimizerScheduler::new(&edges_observer, weighted_scheduler);

    // Create our Fuzzer
    let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);

    // Create the base Executor
    let mut executor = base_executor(opt, &mut shmem_provider);
    // Set a custom exit code to be interpreted as a Crash if configured.
    if let Some(crash_exitcode) = opt.crash_exitcode {
        executor = executor.crash_exitcode(crash_exitcode);
    }

    // Enable autodict if configured
    let mut tokens = Tokens::new();
    if !opt.no_autodict {
        executor = executor.autotokens(&mut tokens);
    };

    // Set a custom directory for the current Input if configured;
    // May be used to provide a ram-disk etc..
    if let Some(cur_input_dir) = &opt.cur_input_dir {
        if opt.harness_input_type.is_none() {
            return Err(Error::illegal_argument(
                "cannot use AFL_TMPDIR with stdin input type.",
            ));
        }
        executor = executor.arg_input_file(cur_input_dir.join(get_unique_std_input_file()));
    }

    // Finalize and build our Executor
    let mut executor = executor
        .build(tuple_list!(time_observer, edges_observer))
        .unwrap();

    // Load our seeds.
    if state.must_load_initial_inputs() {
        state
            .load_initial_inputs(
                &mut fuzzer,
                &mut executor,
                &mut restarting_mgr,
                &[fuzzer_dir.join("queue")],
            )
            .unwrap_or_else(|err| panic!("Failed to load initial corpus! {err:?}"));
        println!("We imported {} inputs from disk.", state.corpus().count());
    }

    // We set IsInitialCorpusEntry as metadata for all initial testcases.
    // Used in Cmplog stage if AFL_CMPLOG_ONLY_NEW.
    if opt.cmplog_only_new {
        for id in state.corpus().ids() {
            let testcase = state.corpus().get(id).expect("should be present in Corpus");
            testcase
                .borrow_mut()
                .add_metadata(IsInitialCorpusEntryMetadata {});
        }
    }

    // Add the tokens to State
    state.add_metadata(tokens);

    // Set the start time of our Fuzzer
    *state.start_time_mut() = current_time();

    // Tell [`SeedFeedback`] that we're done loading seeds; rendering it benign.
    fuzzer.feedback_mut().done_loading_seeds();

    // Create a AFLStatsStage; TODO builder?
    let afl_stats_stage = AflStatsStage::new(opt, fuzzer_dir.clone());

    // Set LD_PRELOAD (Linux) && DYLD_INSERT_LIBRARIES (OSX) for target.
    if let Some(preload_env) = &opt.afl_preload {
        std::env::set_var("LD_PRELOAD", preload_env);
        std::env::set_var("DYLD_INSERT_LIBRARIES", preload_env);
    }

    // Create a CmpLog executor if configured.
    if let Some(ref cmplog_binary) = opt.cmplog_binary {
        // The CmpLog map shared between the CmpLog observer and CmpLog executor
        let mut cmplog_shmem = shmem_provider.uninit_on_shmem::<AFLppCmpLogMap>().unwrap();

        // Let the Forkserver know the CmpLog shared memory map ID.
        cmplog_shmem.write_to_env("__AFL_CMPLOG_SHM_ID").unwrap();
        let cmpmap = unsafe { OwnedRefMut::from_shmem(&mut cmplog_shmem) };

        // Create the CmpLog observer.
        let cmplog_observer = AFLppCmpLogObserver::new("cmplog", cmpmap, true);
        let cmplog_ref = cmplog_observer.handle();

        // Create the CmpLog executor.
        // Cmplog has 25% execution overhead so we give it double the timeout
        let cmplog_executor = base_executor(opt, &mut shmem_provider)
            .timeout(Duration::from_millis(opt.hang_timeout * 2))
            .program(cmplog_binary)
            .build(tuple_list!(cmplog_observer))
            .unwrap();

        // Create the CmpLog tracing stage.
        let tracing = AFLppCmplogTracingStage::new(cmplog_executor, cmplog_ref);

        // Create a randomic Input2State stage
        let rq = MultiMutationalStage::new(AFLppRedQueen::with_cmplog_options(true, true));

        // Create an IfStage and wrap the CmpLog stages in it.
        // We run cmplog on the second fuzz run of the testcase.
        // This stage checks if the testcase has been fuzzed more than twice, if so do not run cmplog.
        // We also check if it is an initial corpus testcase
        // and if run with AFL_CMPLOG_ONLY_NEW, then we avoid cmplog.
        let cb = |_fuzzer: &mut _,
                  _executor: &mut _,
                  state: &mut StdState<_, OnDiskCorpus<_>, _, _>,
                  _event_manager: &mut _|
         -> Result<bool, Error> {
            let testcase = state.current_testcase()?;
            if opt.cmplog_only_new && testcase.has_metadata::<IsInitialCorpusEntryMetadata>() {
                return Ok(false);
            }
            Ok(testcase.scheduled_count() == 1)
        };
        let cmplog = IfStage::new(cb, tuple_list!(colorization, tracing, rq));

        // The order of the stages matter!
        let mut stages = tuple_list!(calibration, cmplog, power, afl_stats_stage);

        // Run our fuzzer; WITH CmpLog
        run_fuzzer_with_stage!(
            &opt,
            fuzzer,
            &mut stages,
            &mut executor,
            &mut state,
            &mut restarting_mgr
        );
    } else {
        // The order of the stages matter!
        let mut stages = tuple_list!(calibration, power, afl_stats_stage);

        // Run our fuzzer; NO CmpLog
        run_fuzzer_with_stage!(
            &opt,
            fuzzer,
            &mut stages,
            &mut executor,
            &mut state,
            &mut restarting_mgr
        );
    }
    Ok(())
    // TODO: serialize state when exiting.
}

fn base_executor<'a>(
    opt: &'a Opt,
    shmem_provider: &'a mut StdShMemProvider,
) -> ForkserverExecutorBuilder<'a, StdShMemProvider> {
    let mut executor = ForkserverExecutor::builder()
        .program(opt.executable.clone())
        .shmem_provider(shmem_provider)
        .coverage_map_size(opt.map_size)
        .kill_signal(opt.kill_signal)
        .debug_child(opt.debug_child)
        .is_persistent(opt.is_persistent)
        .is_deferred_frksrv(opt.defer_forkserver)
        .min_input_size(opt.min_input_len.unwrap_or(AFL_DEFAULT_INPUT_LEN_MIN))
        .max_input_size(opt.max_input_len.unwrap_or(AFL_DEFAULT_INPUT_LEN_MAX))
        .timeout(Duration::from_millis(opt.hang_timeout));
    if let Some(target_env) = &opt.target_env {
        executor = executor.envs(target_env);
    }
    if let Some(harness_input_type) = &opt.harness_input_type {
        executor = executor.parse_afl_cmdline([harness_input_type]);
    }
    executor
}

pub fn fuzzer_target_mode(opt: &Opt) -> Cow<'static, str> {
    let mut res = String::new();
    if opt.unicorn_mode {
        res = format!("{res}unicorn ");
    }
    if opt.qemu_mode {
        res = format!("{res}qemu ");
    }
    if opt.forkserver_cs {
        res = format!("{res}coresight ");
    }
    if opt.no_forkserver {
        res = format!("{res}no_fsrv ");
    }
    if opt.crash_mode {
        res = format!("{res}crash ");
    }
    if opt.is_persistent {
        res = format!("{res}persistent ");
    }
    // TODO: not always shmem_testcase
    res = format!("{res}shmem_testcase ");
    if opt.defer_forkserver {
        res = format!("{res}deferred ");
    }
    if !(opt.unicorn_mode
        || opt.qemu_mode
        || opt.forkserver_cs
        || opt.non_instrumented_mode
        || opt.no_forkserver
        || opt.crash_mode
        || opt.is_persistent
        || opt.defer_forkserver)
    {
        res = format!("{res}default");
    }
    Cow::Owned(res)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsInitialCorpusEntryMetadata {}
libafl_bolts::impl_serdeany!(IsInitialCorpusEntryMetadata);

pub type LibaflFuzzState =
    StdState<BytesInput, OnDiskCorpus<BytesInput>, StdRand, OnDiskCorpus<BytesInput>>;
