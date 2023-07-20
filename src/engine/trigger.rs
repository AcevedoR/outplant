use serde::{Deserialize, Serialize};

use crate::engine::state::State;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Trigger {
    pub(crate) comparator: TriggerComparator,
    pub(crate) target: TriggerTarget,
    pub(crate) value: u32,
}

impl Trigger {
    pub fn is_satisfied(&self, state: &State) -> bool {
        let actual_value = match self.target {
            TriggerTarget::Population => state.population,
            TriggerTarget::Ecology => state.ecology,
            TriggerTarget::Money => state.money,
        };
        return match self.comparator {
            TriggerComparator::Lt => actual_value < self.value,
            TriggerComparator::Lte => actual_value <= self.value,
            TriggerComparator::Eq => actual_value == self.value,
            TriggerComparator::Gte => actual_value >= self.value,
            TriggerComparator::Gt => actual_value > self.value,
        };
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum TriggerComparator {
    Lt,
    Lte,
    Eq,
    Gte,
    Gt,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum TriggerTarget {
    Population,
    Ecology,
    Money,
}
