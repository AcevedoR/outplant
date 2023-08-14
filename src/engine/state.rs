use serde::{Deserialize, Serialize};

use crate::log;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub population: u32,
    pub ecology: u32,
    pub money: u32,
    pub turn_counter: u32,
}

impl State {
    pub fn new(population: u32, ecology: u32, money: u32) -> State {
        let mut instance = State {
            population: 0,
            ecology: 0,
            money: 0,
            turn_counter: 1,
        };
        instance.set_population(population);
        instance.set_ecology(ecology);
        instance.set_money(money);
        return instance;
    }

    fn set_population(&mut self, new: u32) {
        if new > 12 {
            panic!();
        }
        self.population = new;
    }

    fn set_ecology(&mut self, new: u32) {
        if new > 12 {
            panic!();
        }
        self.ecology = new;
    }

    fn set_money(&mut self, new: u32) {
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

        self.turn_counter += 1;

        log!(format!("state after evolve: {:?}", self));
    }
}
