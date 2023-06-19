use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    pub(crate) text: String,
    pub(crate) effects: Option<HashMap<String, bool>>,
    pub(crate) choices: Option<Vec<Choice>>,
}

impl Event {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Choice {
    pub(crate) text: String,
    pub(crate) next: Vec<ChoiceOutcome>,
}

impl Choice {
    pub fn resolve(&self) -> ChoiceOutcome {
        return self.next.first().unwrap().clone();
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChoiceOutcome {
    pub(crate) event: String,
    // pub(crate) in: u32,
    pub(crate) weight: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventId {
    pub(crate) event_chain_id: String,
    pub(crate) id: String,
}
