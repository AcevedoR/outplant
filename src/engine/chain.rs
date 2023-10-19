use serde::{Deserialize, Serialize};

use crate::engine::trigger::Trigger;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chain {
    pub(crate) title: String,
    pub(crate) cooldown: u32,
    pub(crate) trigger: Option<Trigger>,
    pub(crate) auto_select: bool,
}
