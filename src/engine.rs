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
}

impl Engine {
    pub fn new() -> Engine {
        return Engine {
            turn: 1,
            state: State::new(
                10,
                80,
                1000,
            ),
            event_store: EventStore::new(),
        };
    }
    pub fn play_next_cycle(&mut self) -> Option<EventWrapper> {
        // TODO if population == 0 then Game over
        self.state.evolve();

        // let mut rng = rand::thread_rng();
        // if self.state.natural_balance < 70 && rng.gen_range(0..100) < get_induced_disaster_occurence_percentage(&self.state) {
        //     return Some(self.get_disaster());
        // }
        return self.get_random_event_chain();
    }
    pub fn apply_choice(&mut self, choice_wrapper: &ChoiceWrapper) -> ChoiceOutcome{
        let choice_outcome = choice_wrapper.choice.resolve();
        self.schedule_next_event_in_chain(EventId { event_chain_id: choice_wrapper.clone().event_chain_id, id: choice_outcome.event.clone() }, 1);
        return choice_outcome.clone();
    }
    pub fn schedule_next_event_in_chain(&mut self, event_id: EventId, delay: u8) {
        let next_event = self.event_store.getEvent(event_id);// TODO we need chain id everywhere..
    }
    pub fn apply_effect(&mut self, effect: &Effect) {
        // match effect.target {
        //     ChangeTarget::Population => &self.state.set_population(apply_operation_on_state_value(self.state.population, change.value, change.operation.clone())),
        //     ChangeTarget::NaturalBalance => &self.state.set_natural_balance(apply_operation_on_state_value(self.state.natural_balance as u32, change.value, change.operation.clone()) as u8),
        //     ChangeTarget::ExternalInterventionStock => &self.state.set_external_intervention_reserve(apply_operation_on_state_value(self.state.external_intervention_reserve, change.value, change.operation.clone()))
        // }; TODO
    }

    pub fn get_state(&self) -> &State {
        return &self.state;
    }

    fn get_random_event_chain(&self) -> Option<EventWrapper> {
        if rand::thread_rng().gen_range(0..100) < 70 {
            return None;
        }
        let res = self.event_store.event_chains.iter().filter(|event| !!!event.is_disaster()).choose(&mut rand::thread_rng());
        if res.is_none() {
            panic!("no event found in {:?}", self.event_store.event_chains);
        }
        let chain = res.cloned().unwrap();
        return Some(EventWrapper{event_chain_id: chain.title, event: chain.events.get("start").unwrap().clone()});
    }
}

fn apply_operation_on_state_value(actual_value: u32, modifier: f64, operation: ChangeOperation) -> u32 {
    (match operation {
        ChangeOperation::ADD => actual_value as f64 + modifier,
        ChangeOperation::MULTIPLY => actual_value as f64 * modifier,
    }) as u32
}

fn get_induced_disaster_occurence_percentage(state: &State) -> u8 {
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