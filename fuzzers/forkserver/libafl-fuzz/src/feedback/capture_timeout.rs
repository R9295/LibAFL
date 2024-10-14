use std::{borrow::Cow, fmt::Debug};

use libafl::{
    corpus::Testcase,
    executors::ExitKind,
    feedbacks::{Feedback, StateInitializer},
    state::HasCorpus,
    HasMetadata,
};
use libafl_bolts::{Error, Named};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::stages::verify_timeouts::TimeoutsToVerify;

#[derive(Serialize, Deserialize)]
pub struct CaptureTimeoutFeedback {}

impl CaptureTimeoutFeedback {
    /// Create a new [`CaptureTimeoutFeedback`].
    pub fn new() -> Self {
        Self {}
    }
}

impl Named for CaptureTimeoutFeedback {
    fn name(&self) -> &Cow<'static, str> {
        static NAME: Cow<'static, str> = Cow::Borrowed("CaptureTimeoutFeedback");
        &NAME
    }
}

impl<S> StateInitializer<S> for CaptureTimeoutFeedback {}

impl<EM, I, OT, S> Feedback<EM, I, OT, S> for CaptureTimeoutFeedback
where
    S: HasCorpus + HasMetadata,
    I: Debug + Serialize + DeserializeOwned + Default + 'static + Clone,
{
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    fn is_interesting(
        &mut self,
        state: &mut S,
        _manager: &mut EM,
        input: &I,
        _observers: &OT,
        exit_kind: &ExitKind,
    ) -> Result<bool, Error> {
        if matches!(exit_kind, ExitKind::Timeout) {
            let timeouts = state.metadata_or_insert_with(|| TimeoutsToVerify::<I>::new());
            timeouts.push(input.clone());
        }
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

    #[cfg(feature = "track_hit_feedbacks")]
    #[inline]
    fn last_result(&self) -> Result<bool, Error> {
        Ok(false)
    }
}