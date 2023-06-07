// use std::fs::File;
// use std::io::Read;

use std::fmt::format;

use serde_json::json;

use gloo_console::log;
use serde::{Deserialize, Serialize};

use crate::event::{Event, EventId};
use crate::event_chain::EventChain;

use crate::embed::get_event_chains;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventStore {
    pub event_chains: Vec<EventChain>,
}

impl EventStore {
    pub fn new() -> EventStore {
        let chains = get_event_chains();
        log!(format!("CHAINS {} {}", chains.len(), serde_json::to_string_pretty(&chains).unwrap()));

        return EventStore {
            event_chains: chains ,
        };
    }
    pub fn get_event(&self, event_id: EventId) -> &Event {
        return self.get_event2(event_id.event_chain_id, event_id.id);
    }
    pub fn get_event2(&self, event_chain_id: String, event_id: String) -> &Event {
        return self
            .event_chains
            .iter()
            .filter(|event_chain| event_chain.title == event_chain_id)
            .next()
            .unwrap()
            .events
            .get(&*event_id)
            .unwrap();
    }
}
