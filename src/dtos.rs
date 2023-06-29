use serde::{Deserialize, Serialize};

use crate::event::{Choice, Event};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChoiceWrapper {
    pub(crate) choice: Choice,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OngoingEventChain {
    pub(crate) event: Event,
    pub(crate) event_chain_id: String,
}
