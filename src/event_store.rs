// use std::fs::File;
// use std::io::Read;

use serde::{Deserialize, Serialize};

use crate::event::Event;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventStore {
    pub events: Vec<Event>,
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
        let events: Vec<Event> = result.unwrap();

        return EventStore {
            events
        };
    }
}