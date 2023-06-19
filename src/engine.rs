use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::dtos::{ChoiceWrapper, OngoingEventChain};
use crate::event::{ChoiceOutcome, EventId};
use crate::event_store::EventStore;
use crate::state::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Engine {
    turn: u32,
    state: State,
    event_store: EventStore,
    // current_effects: Map<String, Effect>,
}

impl Engine {
    pub fn new() -> Engine {
        return Engine {
            turn: 1,
            state: State::new(1, 12, 1000),
            event_store: EventStore::new(),
        };
    }
    pub fn play_next_cycle(&mut self) {
        if self.has_won() {
            println!("Bravo !");
            panic!("you won");
        }
        if self.has_lost() {
            println!("Looser! Not bravo");
            panic!("you lost")
        }

        self.state.evolve();

        let mut new_event_chains = self.select_event_chains();

        while !self.state.ongoing_event_chains.is_empty() {
            new_event_chains.push(self.state.ongoing_event_chains.pop().unwrap())
        }

        for event_chain in &new_event_chains {
            // if event_chain.event. TODO
        }
    }

    fn has_lost(&self) -> bool {
        return self.state.money <= 0 ||
            self.state.population == 0;
    }

    fn has_won(&self) -> bool {
        self.state.population == 8
    }

    pub fn apply_choice(&mut self, choice_wrapper: &ChoiceWrapper) -> ChoiceOutcome {
        let choice_outcome = choice_wrapper.choice.resolve();

        self.schedule_next_event_in_chain(
            EventId {
                event_chain_id: choice_wrapper.clone().event_chain_id,
                id: choice_outcome.event.clone(),
            },
            1,
        );
        return choice_outcome.clone();
    }

    pub fn schedule_next_event_in_chain(&mut self, event_id: EventId, _delay: u32) {
        let _event = self.event_store.get_event(event_id);
        // TODO implement delay
    }

    fn select_event_chains(&self) -> Vec<OngoingEventChain> {
        return self
            .event_store
            .event_chains
            .iter()
            .filter(|chain| chain.trigger.is_satisfied(&self.state))
            .filter(|chain| !!!self.state.ongoing_event_chains.iter()
                .find(|e| e.event_chain_id == chain.title)
                .is_some()
            )
            .filter(|_chain| rand::thread_rng().gen_range(0..100) < 80)
            .map(|chain| OngoingEventChain {
                event_chain_id: chain.title.clone(),
                event: chain.events.get("start").unwrap().clone(),
            })
            .collect::<Vec<OngoingEventChain>>();
    }

    pub fn get_state(&self) -> &State {
        return &self.state;
    }
}
