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
        return EventStore {
            event_chains: get_event_chains(),
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
