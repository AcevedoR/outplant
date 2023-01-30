use rand::prelude::IteratorRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::event::{Change, ChangeOperation, ChangeTarget, Choice, Event};
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
    pub fn play_next_cycle(&mut self) -> Option<Event> {
        // TODO if population == 0 then Game over
        self.state.evolve();

        let mut rng = rand::thread_rng();
        if self.state.natural_balance < 70 && rng.gen_range(0..100) < get_induced_disaster_occurence_percentage(&self.state) {
            return Some(self.get_disaster());
        }
        return self.get_random_event();
    }
    pub fn apply_choice(&mut self, choice: &Choice) -> Vec<Change> {
        let changes = choice.resolve();
        for change in &changes {
            match change.target {
                ChangeTarget::Population => &self.state.set_population(apply_operation_on_state_value(self.state.population, change.value, change.operation.clone())),
                ChangeTarget::NaturalBalance => &self.state.set_natural_balance(apply_operation_on_state_value(self.state.natural_balance as u32, change.value, change.operation.clone()) as u8),
                ChangeTarget::ExternalInterventionStock => &self.state.set_external_intervention_reserve(apply_operation_on_state_value(self.state.external_intervention_reserve, change.value, change.operation.clone()))
            };
        }
        return changes;
    }
    pub fn get_state(&self) -> &State {
        return &self.state;
    }

    fn get_disaster(&self) -> Event {
        return self.event_store.events.iter().filter(|event| event.is_disaster()).choose(&mut rand::thread_rng()).unwrap().clone();
    }

    fn get_random_event(&self) -> Option<Event> {
         if rand::thread_rng().gen_range(0..100) < 70 {
             return None;
         }
        let res = self.event_store.events.iter().filter(|event| !!!event.is_disaster()).choose(&mut rand::thread_rng());
        if res.is_none() {
            panic!("no event found in {:?}", self.event_store.events);
        }
        return res.cloned();
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