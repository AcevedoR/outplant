use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::effect::Effect;
use crate::state::State;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    pub(crate) text: String,
    pub(crate) effects: Option<HashMap<String, bool>>,
    pub(crate) choices: Option<Vec<Choice>>,
}

impl Event {
    pub fn apply_effects(self: &Event, state: &mut State, event_chain_effects: &HashMap<String, Effect>) {
        if self.effects.is_some() {
            self.effects.as_ref().unwrap().iter()
                .filter(|(_, is_event_effect_activated)| **is_event_effect_activated == true)
                .map(|(event_effect_id, _)| event_chain_effects.get(event_effect_id).unwrap())
                .for_each(|effect| effect.apply(state));
        }
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
    // pub(crate) in: u32,
    pub(crate) weight: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventId {
    pub(crate) event_chain_id: String,
    pub(crate) id: String,
}
