use serde::{Deserialize, Serialize};

use crate::state::State;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Effect {
    pub(crate) description: String,
    pub(crate) operation: ChangeOperation,
    pub(crate) target: ChangeTarget,
    pub(crate) value: u32,
    // pub(crate) type: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ChangeOperation {
    Add,
    Subtract,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ChangeTarget {
    Population,
    Ecology,
    Money,
}

impl Effect {
    pub fn apply(self: &Effect, state: &mut State) {
        let mut value = self.value;
        match self.operation {
            ChangeOperation::Add => {}
            ChangeOperation::Subtract => {
                value = 0 - value;
            }
        }
        match self.target {
            ChangeTarget::Population => {
                state.population = state.population + value;
            }
            ChangeTarget::Ecology => {
                state.ecology = state.ecology + value;
            }
            ChangeTarget::Money => {
                state.money = state.money + value;
            }
        }
    }
}