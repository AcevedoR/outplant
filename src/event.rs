use std::collections::HashMap;
use std::iter::Map;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    pub(crate) text: String,
    pub(crate) effects: Option<HashMap<String, bool>>,
    pub(crate) choices: Option<Vec<Choice>>,
}

impl Event {
    pub(crate) fn is_choice(&self) -> bool {
        self.choices.is_some()
    }
}

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
    // pub(crate) in: u8,
    pub(crate) weight: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventId {
    pub(crate) event_chain_id: String,
    pub(crate) id: String,
}