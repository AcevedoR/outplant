use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::effect::Effect;
use crate::event::Event;
use crate::trigger::Trigger;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventChain {
    pub(crate) title: String,
    pub(crate) cooldown: u32,
    pub(crate) trigger: Trigger,
    pub(crate) events: HashMap<String, Event>,
    pub(crate) effects: HashMap<String, Effect>,
}

impl EventChain {}
