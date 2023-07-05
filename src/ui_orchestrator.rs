use crate::engine::Engine;
use crate::random::PseudoRandomGenerator;
use crate::state::State;

pub struct UIOrchestrator {
    engine: Engine<PseudoRandomGenerator>,
}

impl UIOrchestrator {
    pub fn new() -> UIOrchestrator {
        return UIOrchestrator {
            engine: Engine::new(PseudoRandomGenerator{}),
        };
    }

    pub fn make_choice(&mut self, index: usize) -> ViewModel {
        self.engine.make_choice(index)
    }

    pub fn next_cycle(&mut self) -> ViewModel {
        self.engine.next_cycle()
    }

    pub fn get_state(&self) -> &State {
        self.engine.get_state()
    }
}

#[derive(Clone, Debug)]
pub struct ViewModel {
    pub lines: Vec<String>,
    pub choices: Vec<String>,
}
