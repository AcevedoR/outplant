use serde::{Deserialize, Serialize};
use crate::log_wasm;

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
        let mut value: i32 = self.value as i32;
        log_wasm!(format!("applying effect {:?}", self));
        match self.operation {
            ChangeOperation::Add => {}
            ChangeOperation::Subtract => {
                value = 0 - value;
            }
        }
        match self.target {
            ChangeTarget::Population => {
                state.population = state.population.saturating_add_signed(value);
            }
            ChangeTarget::Ecology => {
                state.ecology = state.ecology.saturating_add_signed(value);
            }
            ChangeTarget::Money => {
                state.money = state.money.saturating_add_signed(value);
            }
        }
    }
}