use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) choices: Vec<Choice>,
}

impl Event {
    pub(crate) fn is_disaster(&self) -> bool {
        self.title == "Famine" || self.title == "Plague" // TODO move to events.json file, as Tags
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Choice {
    pub(crate) description: String,
    pub(crate) possible_changes: Vec<Change>,
}

impl Choice {
    pub fn resolve(&self) -> Vec<Change> {
        return self.possible_changes.clone().clone();
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Change {
    pub(crate) description: String,
    pub(crate) operation: ChangeOperation,
    pub(crate) target: ChangeTarget,
    pub(crate) value: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum ChangeOperation {
    ADD,
    MULTIPLY,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum ChangeTarget {
    Population,
    NaturalBalance,
    ExternalInterventionStock,
}

