use std::collections::HashSet;
use gloo_console::log;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub(crate) population: u32,
    pub(crate) natural_balance: u32,
    pub(crate) external_intervention_reserve: u32,
    pub(crate) ongoing_event_chains: HashSet<String>
}

impl State {
    pub fn new(population: u32, natural_balance: u32, external_intervention_reserve: u32) -> State {
        let mut instance = State {
            population: 0,
            natural_balance: 0,
            external_intervention_reserve: 0,
            ongoing_event_chains: Default::default(),
        };
        instance.set_population(population);
        instance.set_natural_balance(natural_balance);
        instance.set_external_intervention_reserve(external_intervention_reserve);
        return instance;
    }
    pub fn set_population(&mut self, new: u32) {
        if new < 0 || new > 12 {
            panic!();
        }
        self.population = new;
    }
    pub fn set_natural_balance(&mut self, new: u32) {
        if new < 0 || new > 12 {
            panic!();
        }
        self.natural_balance = new;
    }
    pub fn set_external_intervention_reserve(&mut self, new: u32) {
        self.external_intervention_reserve = new;
    }

    pub fn evolve(&mut self) {
        if self.population != 0 && self.population < 12 {
            self.population = self.population + 1;
        }

        if self.population == 12 && self.natural_balance > 0 {
            self.natural_balance = self.natural_balance - 1;
        }

        if self.natural_balance == 0 && self.population > 0 {
            self.population = self.population - 1;
        }
        log!("state after evolve: ", JsValue::from(format!("{:?}", self)));

    }
}
