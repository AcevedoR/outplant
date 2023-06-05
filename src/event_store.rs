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
            panic!(
                "events json file was not well formatted, \nerror: {}, \nfile: {}",
                result.err().unwrap(),
                data
            );
        }
        let events: Vec<EventChain> = result.unwrap();

        return EventStore {
            event_chains: events,
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
