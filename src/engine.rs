use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::dtos::{ChoiceWrapper, EventWrapper};
use crate::event::{ChoiceOutcome, EventId};
use crate::event_store::EventStore;
use crate::state::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Engine {
    turn: u32,
    state: State,
    event_store: EventStore,
}

impl Engine {
    pub fn new() -> Engine {
        return Engine {
            turn: 1,
            state: State::new(1, 12, 1000),
            event_store: EventStore::new(),
        };
    }
    pub fn play_next_cycle(&mut self) -> Vec<EventWrapper> {
        if self.has_won() {
            println!("Bravo !");
            panic!("you won");
        }
        if self.has_lost() {
            println!("Looser! Not bravo");
            panic!("you lost")
        }

        self.state.evolve();

        let new_event_chains = self.select_event_chain();

        for chain in &new_event_chains {
            self.state
                .ongoing_event_chains
                .insert(chain.clone().event_chain_id);
        }

        return new_event_chains;
    }

    fn has_lost(&self) -> bool {
        return self.state.external_intervention_reserve <= 0 ||
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

    fn select_event_chain(&self) -> Vec<EventWrapper> {
        return self
            .event_store
            .event_chains
            .iter()
            .filter(|chain| chain.trigger.is_satisfied(&self.state))
            .filter(|chain| !!!self.state.ongoing_event_chains.contains(&chain.title))
            .filter(|_chain| rand::thread_rng().gen_range(0..100) < 80)
            .map(|chain| EventWrapper {
                event_chain_id: chain.title.clone(),
                event: chain.events.get("start").unwrap().clone(),
            })
            .collect::<Vec<EventWrapper>>();
    }

    fn play_event(&self, _event: &EventWrapper) {
        // TODO
    }

    pub fn get_state(&self) -> &State {
        return &self.state;
    }
}
