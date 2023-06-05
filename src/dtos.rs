use serde::{Deserialize, Serialize};

use crate::event::{Choice, Event};

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
