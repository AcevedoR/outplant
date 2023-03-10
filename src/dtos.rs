use crate::event::{Choice, Event};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChoiceWrapper {
    pub(crate) choice: Choice,
    pub(crate) event_chain_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventWrapper {
    pub(crate) event: Event,
    pub(crate) event_chain_id: String,
}

