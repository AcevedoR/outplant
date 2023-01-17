use gloo_console::log;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub(crate) population: u32,
    pub(crate) natural_balance: u8,
    pub(crate) external_intervention_reserve: u32,
    population_evolution: PopulationEvolution,
}

impl State {
    pub fn new(population: u32, natural_balance: u8, external_intervention_reserve: u32) -> State {
        let mut instance = State {
            population: 0,
            natural_balance: 0,
            external_intervention_reserve: 0,
            population_evolution: PopulationEvolution { old_population: 0, new_population: 0 },
        };
        instance.set_population(population);
        instance.set_natural_balance(natural_balance);
        instance.set_external_intervention_reserve(external_intervention_reserve);
        return instance;
    }
    pub fn set_population(&mut self, new: u32) {
        if new < 0 {
            panic!();
        }
        self.population_evolution.set(new);
        self.population = new;
    }
    pub fn set_natural_balance(&mut self, new: u8) {
        if new < 0 || new > 100 {
            panic!();
        }
        self.natural_balance = new;
    }
    pub fn set_external_intervention_reserve(&mut self, new: u32) {
        self.external_intervention_reserve = new;
    }
    pub fn evolve(&mut self) {
        self.population_evolution.reset(self.population);

        let new_population = (self.population as f32 * 1.2 * ((self.natural_balance + 20) as f32 * 0.01)).floor() as u32;
        self.set_population(new_population);

        let population_increase_ratio = self.population_evolution.get_evolution_rate();

        log!("Hello", JsValue::from(format!("{:?}", self)));

        if population_increase_ratio > 0.2 {
            self.set_natural_balance(self.natural_balance - (30.0 * population_increase_ratio) as u8)
        } else if self.natural_balance < 80 {
            self.set_natural_balance(self.natural_balance + 1)
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PopulationEvolution {
    old_population: u32,
    new_population: i32,
}

impl PopulationEvolution {
    pub(crate) fn set(&mut self, new: u32) {
        if self.old_population == 0 {
            self.old_population = new;
        } else {
            self.new_population = self.new_population - self.old_population as i32 + new as i32;
        }
    }
    pub(crate) fn get_evolution_rate(&self) -> f32 {
        self.new_population as f32 / self.old_population as f32
    }
    pub(crate) fn reset(&mut self, current_population: u32) {
        self.old_population = current_population;
        self.new_population = 0;
    }
}