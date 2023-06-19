use serde::{Deserialize, Serialize};

use crate::dtos::{ChoiceWrapper, OngoingEventChain};
use crate::engine::Engine;
use crate::event::ChoiceOutcome;
use crate::state::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct UIOrchestrator {
    engine: Engine,
}

impl UIOrchestrator {
    pub fn new() -> UIOrchestrator {
        return UIOrchestrator {
            engine: Engine::new(),
        };
    }
    pub fn make_a_choice(&mut self, choice: &ChoiceWrapper) -> ChoiceOutcome {
        return self.engine.apply_choice(choice);
    }
    pub fn play_next_cycle(&mut self) -> Vec<OngoingEventChain> {
        self.engine.play_next_cycle();
        return self.engine.get_state().clone().ongoing_event_chains;
    }
    pub fn get_state(&self) -> &State {
        self.engine.get_state()
    }
}
