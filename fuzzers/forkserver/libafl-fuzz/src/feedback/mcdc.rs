use std::{
    borrow::Cow,
    path::{PathBuf},
    process::Command,
};

use libafl::{
    corpus::Testcase,
    executors::ExitKind,
    feedbacks::{Feedback, FeedbackFactory, StateInitializer},
};
use libafl_bolts::{core_affinity::CoreId, Error, Named};
use serde::{Deserialize, Serialize};

/// A [`MCDCFeedback`] takes a closure which can set the file name and path for the testcase.
/// Is never interesting (use with an OR).
/// Note: If used as part of the `Objective` chain, then it will only apply to testcases which are
/// `Objectives`, vice versa for `Feedback`.
#[derive(Serialize, Deserialize)]
pub struct MCDCFeedback {
    covered: f32,
    core_id: usize,
    bin_path: PathBuf,
}

impl std::fmt::Debug for MCDCFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    where
        Self: Sized,
    {
        f.debug_struct("MCDCFeedback")
            .field("covered", &self.covered)
            .finish_non_exhaustive()
    }
}

impl MCDCFeedback {
    /// Create a new [`MCDCFeedback`].
    pub fn new(core_id: CoreId, bin_path: PathBuf) -> Self {
        Self {
            covered: 0.0,
            core_id: core_id.0,
            bin_path,
        }
    }
}

impl<T> FeedbackFactory<MCDCFeedback, T> for MCDCFeedback {
    fn create_feedback(&self, _ctx: &T) -> MCDCFeedback {
        Self {
            covered: 0.0,
            core_id: 0,
            bin_path: PathBuf::default(),
        }
    }
}

impl Named for MCDCFeedback {
    fn name(&self) -> &Cow<'static, str> {
        static NAME: Cow<'static, str> = Cow::Borrowed("MCDCFeedback");
        &NAME
    }
}

impl<S> StateInitializer<S> for MCDCFeedback {}

impl<EM, I, OT, S> Feedback<EM, I, OT, S> for MCDCFeedback {
    #[inline]
    fn is_interesting(
        &mut self,
        _state: &mut S,
        _manager: &mut EM,
        _input: &I,
        _observers: &OT,
        _exit_kind: &ExitKind,
    ) -> Result<bool, Error> {
        if self.core_id % 3 != 0 {
            return Ok(false);
        }
        let profrawfile = format!("/dev/shm/mcdc.{}.profraw", self.core_id);
        let profdatafile = format!("/dev/shm/mcdc.{}.profdata", self.core_id);
        if std::fs::exists(&profdatafile)? {
            Command::new("llvm-profdata-21")
                .arg("merge")
                .arg("-sparse")
                .arg(profrawfile)
                .arg(&profdatafile)
                .arg("-o")
                .arg(&profdatafile)
                .output()?;
        } else {
            Command::new("llvm-profdata-21")
                .arg("merge")
                .arg("-sparse")
                .arg(&profrawfile)
                .arg("-o")
                .arg(&profdatafile)
                .output()?;
        }
        let output = Command::new("llvm-cov-21")
            .arg("report")
            .arg("--show-mcdc-summary")
            .arg("-instr-profile")
            .arg(&profdatafile)
            .arg(&self.bin_path)
            .output()?
            .stdout;
        let output_lines = std::str::from_utf8(&output)?
            .split("\n")
            .collect::<Vec<_>>();
        let mcdc = output_lines
            .get(output_lines.len() - 2)
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .replace("%", "")
            .parse::<f32>()
            .unwrap();
        if mcdc > self.covered {
            println!("{} -> {:?} ---> {:?}", self.core_id, self.covered, mcdc);
            self.covered = mcdc;
            return Ok(true);
        }
        self.covered = mcdc;

        Ok(false)
    }

    #[cfg(feature = "track_hit_feedbacks")]
    #[inline]
    fn last_result(&self) -> Result<bool, Error> {
        Ok(false)
    }

    fn append_metadata(
        &mut self,
        _state: &mut S,
        _manager: &mut EM,
        _observers: &OT,
        _testcase: &mut Testcase<I>,
    ) -> Result<(), Error> {
        Ok(())
    }
}
