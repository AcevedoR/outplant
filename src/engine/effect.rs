use serde::{Deserialize, Serialize};
use crate::log;

use crate::engine::state::State;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Effect {
    pub(crate) description: Option<String>,
    pub(crate) operation: ChangeOperation,
    pub(crate) target: ChangeTarget,
    pub(crate) value: u32,
    #[serde(rename = "type")]
    pub(crate) effect_type: EffectType,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum EffectType {
    Instant,
    Permanent,
}

impl Effect {
    pub fn apply(self: &Effect, state: &mut State) {
        let mut value: i32 = self.value as i32;
        log!(format!("applying effect {:?}", self));
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
