use serde::{Deserialize, Serialize};

use crate::embed::get_event_chains;
use crate::event_chain::EventChain;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventStore {
    pub event_chains: Vec<EventChain>,
}

impl EventStore {
    pub fn new() -> EventStore {
        return EventStore {
            event_chains: get_event_chains(),
        };
    }
}
