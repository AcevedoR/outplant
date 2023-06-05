use std::collections::HashMap;
use std::iter::Map;

use serde::{Deserialize, Serialize};

use crate::effect::Effect;
use crate::event::Event;
use crate::state::State;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Trigger {
    pub(crate) comparator: TriggerComparator,
    pub(crate) target: TriggerTarget,
    pub(crate) value: u32,
}

impl Trigger {
    pub(crate) fn is_satisfied(&self, state: &State) -> bool {
        let actual_value = match self.target {
            TriggerTarget::Population => {
                state.population
            }
            TriggerTarget::NaturalBalance => {
                state.natural_balance
            }
            TriggerTarget::ExternalInterventionStock => {
                state.external_intervention_reserve
            }
        };
        return match self.comparator {
            TriggerComparator::gt => {
                actual_value > self.value
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum TriggerComparator {
    gt,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum TriggerTarget {
    Population,
    NaturalBalance,
    ExternalInterventionStock,
}