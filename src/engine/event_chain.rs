use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::engine::effect::Effect;
use crate::engine::event::Event;
use crate::engine::trigger::Trigger;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventChain {
    pub(crate) title: String,
    pub(crate) cooldown: u32,
    pub(crate) trigger: Option<Trigger>,
    pub(crate) events: HashMap<String, Event>,
    pub(crate) effects: HashMap<String, Effect>,
}

impl EventChain {}
