use serde::{Deserialize, Serialize};

use crate::dtos::{ChoiceWrapper, EventWrapper};
use crate::engine::Engine;
use crate::event::ChoiceOutcome;
use crate::state::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct UIOrchestrator {
    engine: Engine,
}

impl UIOrchestrator {
    pub fn new() -> UIOrchestrator {
        // let _first_event = Event {
        //     title: "Welcome, fake god..".parse().unwrap(),
        //     description: "Your species was created, and it starting to evolve !".parse().unwrap(),
        //     choices: vec![],
        // };
        return UIOrchestrator {
            engine: Engine::new(),
        };
    }
    pub fn make_a_choice(&mut self, choice: &ChoiceWrapper) -> ChoiceOutcome {
        return self.engine.apply_choice(choice);
    }
    pub fn play_next_cycle(&mut self) -> Vec<EventWrapper> {
        return self.engine.play_next_cycle();
    }
    pub fn get_state(&self) -> &State {
        self.engine.get_state()
    }
}
