use crate::log;
use serde::{Deserialize, Serialize};

use crate::engine::state::State;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Effect {
    pub(crate) description: Option<String>,
    pub(crate) operation: ChangeOperation,
    pub(crate) target: ChangeTarget,
    pub(crate) value: u32,
    #[serde(rename = "type")]
    pub(crate) effect_type: EffectType,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ChangeOperation {
    Add,
    Subtract,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ChangeTarget {
    Population,
    Ecology,
    Money,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub(crate) enum EffectType {
    Instant,
    Permanent,
}

impl Effect {
    pub fn apply(self: &Effect, state: &mut State) {
        log!(format!("applying effect {:?}", self));

        match self.operation {
            ChangeOperation::Add => match self.target {
                ChangeTarget::Population => {
                    state.add_population(self.value);
                }
                ChangeTarget::Ecology => {
                    state.add_ecology(self.value);
                }
                ChangeTarget::Money => {
                    state.add_money(self.value);
                }
            },
            ChangeOperation::Subtract => match self.target {
                ChangeTarget::Population => {
                    state.subtract_population(self.value);
                }
                ChangeTarget::Ecology => {
                    state.subtract_ecology(self.value);
                }
                ChangeTarget::Money => {
                    state.subtract_money(self.value);
                }
            },
        }
    }
}
