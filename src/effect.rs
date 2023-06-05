use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Effect {
    pub(crate) description: String,
    pub(crate) operation: ChangeOperation,
    pub(crate) target: ChangeTarget,
    pub(crate) value: f64,
    // pub(crate) type: f64,
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
