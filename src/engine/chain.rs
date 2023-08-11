use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::engine::{effect::Effect, event::Event, trigger::Trigger};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chain {
    pub(crate) title: String,
    pub(crate) cooldown: u32,
    pub(crate) trigger: Option<Trigger>,
    pub(crate) events: HashMap<String, Event>,
    pub(crate) effects: HashMap<String, Effect>,
}

impl Chain {}
