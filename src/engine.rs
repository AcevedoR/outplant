use std::thread::current;

use rand::prelude::IteratorRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::dtos::{ChoiceWrapper, EventWrapper};
use crate::effect::{ChangeOperation, ChangeTarget, Effect};
use crate::event::{Choice, ChoiceOutcome, Event, EventId};
use crate::event_chain::EventChain;
use crate::event_store::EventStore;
use crate::state::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Engine {
    turn: u32,
    state: State,
    event_store: EventStore,
    next_event: Option<EventWrapper>,
}

impl Engine {
    pub fn new() -> Engine {
        return Engine {
            turn: 1,
            state: State::new(
                1,
                12,
                1000,
            ),
            event_store: EventStore::new(),
            next_event: None,
        };
    }
    pub fn play_next_cycle(&mut self) -> Option<EventWrapper> {
        if self.has_won(self.state.external_intervention_reserve, self.state.population, self.state.natural_balance) {
            println!("Bravo !");
            panic!("you won");
        }
        if self.has_lost(self.state.external_intervention_reserve, self.state.population, self.state.natural_balance) {
            println!("Looser! Not bravo");
            panic!("you lost")
        }

        // if let Some(event_to_play) = &self.next_event {
        //     self.play_event(event_to_play)
        // } TODO event chain

        self.state.evolve();

        let optional_event_chain = self.select_event_chain();
        if optional_event_chain.is_some() {
            self.state.ongoing_event_chains.insert(optional_event_chain.clone().unwrap().event_chain_id);
        }
        return optional_event_chain;
    }

    fn has_lost(&self, money: u32, pop_level: u32, ecology_level: u32) -> bool {
        return money <= 0 ||
            pop_level == 0;
    }

    fn has_won(&self, money: u32, pop_level: u32, ecology_level: u32) -> bool {
        pop_level == 8
    }

    pub fn apply_choice(&mut self, choice_wrapper: &ChoiceWrapper) -> ChoiceOutcome {
        let choice_outcome = choice_wrapper.choice.resolve();

        self.schedule_next_event_in_chain(EventId { event_chain_id: choice_wrapper.clone().event_chain_id, id: choice_outcome.event.clone() }, 1);
        return choice_outcome.clone();
    }
    pub fn schedule_next_event_in_chain(&mut self, event_id: EventId, delay: u32) {
        let event = self.event_store.getEvent(event_id);
        // TODO implement delay
    }
    pub fn apply_effect(&mut self, effect: &Effect) {
        match effect.target {
            ChangeTarget::Population => &self.state.set_population(apply_operation_on_state_value(self.state.population, effect.value, effect.operation.clone())),
            ChangeTarget::NaturalBalance => &self.state.set_natural_balance(apply_operation_on_state_value(self.state.natural_balance as u32, effect.value, effect.operation.clone()) as u32),
            ChangeTarget::ExternalInterventionStock => &self.state.set_external_intervention_reserve(apply_operation_on_state_value(self.state.external_intervention_reserve, effect.value, effect.operation.clone()))
        };
    }

    pub fn get_state(&self) -> &State {
        return &self.state;
    }

    fn select_event_chain(&self) -> Option<EventWrapper> {
        let random_event = self.event_store.event_chains.iter()
            .filter(|chain| chain.trigger.is_satisfied(&self.state))
            .filter(|chain| !!!self.state.ongoing_event_chains.contains(&chain.title))
            .choose(&mut rand::thread_rng());

        if random_event.is_none() {
            return None;
        }

        let chain = random_event.cloned().unwrap();
        let first_event = EventWrapper { event_chain_id: chain.title, event: chain.events.get("start").unwrap().clone() };
        self.play_event(&first_event);
        return Some(first_event);
    }

    fn play_event(&self, event: &EventWrapper) {
        // TODO
    }
}

fn apply_operation_on_state_value(actual_value: u32, modifier: f64, operation: ChangeOperation) -> u32 {
    (match operation {
        ChangeOperation::ADD => actual_value as f64 + modifier,
        ChangeOperation::MULTIPLY => actual_value as f64 * modifier,
    }) as u32
}

fn get_induced_disaster_occurence_percentage(state: &State) -> u32 {
    return 100 - state.natural_balance;
}

#[cfg(test)]
mod tests {
    use crate::engine::get_induced_disaster_occurence_percentage;
    use crate::state::State;

    #[test]
    fn occurence_percentage_none() {
        assert_eq!(get_induced_disaster_occurence_percentage(
            &State::new(
                10,
                100,
                1000,
            )),
                   0
        );
    }

    #[test]
    fn occurence_percentage_almost_always() {
        assert_eq!(get_induced_disaster_occurence_percentage(
            &State::new(
                10,
                10,
                1000,
            )),
                   90
        );
    }

    #[test]
    fn occurence_percentage_sometimes() {
        assert_eq!(get_induced_disaster_occurence_percentage(
            &State::new(
                10,
                60,
                1000,
            )),
                   40
        );
    }
}