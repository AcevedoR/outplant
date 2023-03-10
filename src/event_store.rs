// use std::fs::File;
// use std::io::Read;

use serde::{Deserialize, Serialize};

use crate::event::{Event, EventId};
use crate::event_chain::EventChain;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventStore {
    pub event_chains: Vec<EventChain>,
}

impl EventStore {
    pub fn new() -> EventStore {
        // let mut file = File::open("events.json").unwrap(); TODO fix
        let path: &'static str = env!("UNNAMED_GAME_EVENTS_JSON");
        let data: String = path.parse().unwrap();

        let result = serde_json::from_str(&*data);
        if result.is_err() {
            panic!("events json file was not well formatted, \nerror: {}, \nfile: {}", result.err().unwrap(), data);
        }
        let events: Vec<EventChain> = result.unwrap();

        return EventStore {
            event_chains: events
        };
    }
    pub fn getEvent(&self, eventId: EventId) -> &Event {
        return self.getEvent2(eventId.event_chain_id, eventId.id);
    }
    pub fn getEvent2(&self, eventChainId: String, eventId: String) -> &Event {
        return self.event_chains.iter().filter(|eventChain| eventChain.title == eventChainId).next().unwrap()
            .events.get(&*eventId).unwrap();
    }
}