use gloo_console::log;
use serde::{Deserialize, Serialize};

use crate::dtos::OngoingEventChain;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub(crate) population: u32,
    pub(crate) ecology: u32,
    pub(crate) money: u32,
    pub(crate) ongoing_event_chains: Vec<OngoingEventChain>,
}

impl State {
    pub fn new(population: u32, natural_balance: u32, external_intervention_reserve: u32) -> State {
        let mut instance = State {
            population: 0,
            ecology: 0,
            money: 0,
            ongoing_event_chains: Default::default(),
        };
        instance.set_population(population);
        instance.set_ecology(natural_balance);
        instance.set_money(external_intervention_reserve);
        return instance;
    }
    pub fn set_population(&mut self, new: u32) {
        if new > 12 {
            panic!();
        }
        self.population = new;
    }
    pub fn set_ecology(&mut self, new: u32) {
        if new > 12 {
            panic!();
        }
        self.ecology = new;
    }
    pub fn set_money(&mut self, new: u32) {
        self.money = new;
    }

    pub fn evolve(&mut self) {
        if self.population != 0 && self.population < 12 {
            self.population = self.population + 1;
        }

        if self.population == 12 && self.ecology > 0 {
            self.ecology = self.ecology - 1;
        }

        if self.ecology == 0 && self.population > 0 {
            self.population = self.population - 1;
        }
        log!(format!("state after evolve: {:?}", self));
    }
}
