use std::cmp::{max, min};

use serde::{Deserialize, Serialize};

use crate::log;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    population: u32,
    ecology: u32,
    money: u32,
    turn_counter: u32,
}

impl State {
    pub fn new(population: u32, ecology: u32, money: u32) -> State {
        State {
            population,
            ecology,
            money,
            turn_counter: 0,
        }
    }

    pub fn population(&self) -> &u32 {
        return &self.population;
    }

    pub fn add_population(&mut self, value: u32) {
        self.population = min(self.population + value, 8);
    }

    pub fn subtract_population(&mut self, value: u32) {
        self.population = max(self.population - value, 0);
    }

    pub fn ecology(&self) -> &u32 {
        return &self.ecology;
    }

    pub fn add_ecology(&mut self, value: u32) {
        self.ecology = min(self.ecology + value, 12);
    }

    pub fn subtract_ecology(&mut self, value: u32) {
        self.ecology = max(self.ecology - value, 0);
    }

    pub fn money(&self) -> &u32 {
        return &self.money;
    }

    pub fn add_money(&mut self, value: u32) {
        self.money = self.money + value;
    }

    pub fn subtract_money(&mut self, value: u32) {
        self.money = max(self.money - value, 0);
    }

    pub fn turn_counter(&self) -> &u32 {
        return &self.turn_counter;
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
