use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::utils;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    pub(crate) text: String,
    pub(crate) next: Option<Vec<Next>>,
    pub(crate) effects: Option<HashMap<String, bool>>,
    pub(crate) choices: Option<Vec<Choice>>,
}

impl Event {
    pub fn set_namespace(&mut self, namespace: String) {
        if let Some( nexts) = &mut self.next {
            for next in nexts {
                next.set_namespace(namespace.clone());
            }
        }

        if let Some(effects) = &mut self.effects {
            self.effects = Some(utils::prefix_all_keys(&mut effects.clone(), &namespace))
        }

        if let Some(choices) = &mut self.choices {
            for choice in choices {
                choice.set_namespace(namespace.clone());
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Choice {
    pub(crate) text: String,
    pub(crate) next: Vec<ChoiceOutcome>,
    pub(crate) effects: Option<HashMap<String, bool>>,
}

impl Choice {
    fn set_namespace(&mut self, namespace: String) {
        for outcome in &mut self.next {
            outcome.set_namespace(namespace.clone());
        }

        if let Some(effects) = &self.effects {
            self.effects = Some(utils::prefix_all_keys(&mut effects.clone(), &namespace))
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ChoiceOutcome {
    pub(crate) event: String,
    #[serde(rename = "in")]
    pub(crate) timer: Option<u32>,
    pub(crate) weight: Option<u32>,
    pub(crate) effects: Option<HashMap<String, bool>>,
}

impl ChoiceOutcome {
    fn set_namespace(&mut self, namespace: String) {
        self.event = namespace.clone() + &self.event;

        if let Some(effects) = &self.effects {
            self.effects = Some(utils::prefix_all_keys(&mut effects.clone(), &namespace))
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Next {
    pub(crate) event: String,
    #[serde(rename = "in")]
    pub(crate) timer: Option<u32>,
    pub(crate) weight: Option<u32>,
    pub(crate) effects: Option<HashMap<String, bool>>,
}

impl Next {
    fn set_namespace(&mut self, namespace: String) {
        self.event = namespace.clone() + &self.event;

        if let Some(effects) = &self.effects {
            self.effects = Some(utils::prefix_all_keys(&mut effects.clone(), &namespace))
        }
    }
}
