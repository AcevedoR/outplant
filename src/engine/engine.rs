use serde::{Deserialize, Serialize};

use crate::engine::{
    event::{ChoiceOutcome, Event, Next},
    event_store::EventStore,
    random::RandomGenerator,
    state::State,
};
use crate::log;

#[derive(Clone, Debug)]
pub struct ViewModel {
    pub lines: Vec<String>,
    pub choices: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OngoingEventChain {
    timer: u32,
    pub(crate) event: Event,
    pub(crate) event_chain_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Engine<Rng> {
    turn: u32,
    state: State,
    event_store: EventStore,
    events_to_resolve_this_turn: Vec<OngoingEventChain>,
    events_to_resolve_later: Vec<OngoingEventChain>,
    random_generator: Rng,
}

impl<Rng: RandomGenerator> Engine<Rng> {
    pub fn new(event_chains_filepath: &str, random_generator: Rng) -> Engine<Rng> {
        return Engine {
            turn: 1,
            state: State::new(1, 12, 1000),
            event_store: EventStore::new(event_chains_filepath),
            events_to_resolve_this_turn: Default::default(),
            events_to_resolve_later: Default::default(),
            random_generator,
        };
    }

    pub fn next_cycle(&mut self) -> ViewModel {
        if !self.events_to_resolve_this_turn.is_empty() {
            panic!("next_cycle called when some events of current turn were waiting to be resolved")
        }

        // Test for win and lose condition
        if self.has_won() {
            log!("Bravo!");
            panic!("you won");
        }
        if self.has_lost() {
            log!("Looser! Not bravo");
            panic!("you lost");
        }

        // Apply "natural" effects
        self.state.evolve();

        // Queue new event chains
        let mut new_event_chains = self.select_event_chains();
        while !!!new_event_chains.is_empty() {
            self.events_to_resolve_this_turn
                .push(new_event_chains.pop().unwrap())
        }

        // Queue ongoing event chains
        while !!!self.events_to_resolve_later.is_empty() {
            self.events_to_resolve_this_turn
                .push(self.events_to_resolve_later.pop().unwrap())
        }

        return self.unstack_events_to_resolve_this_turn();
    }

    pub fn make_choice(&mut self, index: usize) -> ViewModel {
        if self.events_to_resolve_this_turn.is_empty() {
            panic!("make_choice called when no events of current turn were waiting to be resolved")
        }

        let event_with_choice = self.events_to_resolve_this_turn.pop().unwrap().event;
        let binding = event_with_choice.choices.unwrap();
        let next = binding.get(index).unwrap();

        let outcome = self.select_choice_outcome(&next.next);

        let next_event = self
            .event_store
            .clone()
            .get_event(outcome.clone().event)
            .unwrap();
        let chain_of_next_event = self.event_store.clone().get_chain(outcome.event).unwrap();
        self.events_to_resolve_this_turn.push(OngoingEventChain {
            timer: outcome.timer.unwrap_or_default(),
            event: next_event,
            event_chain_id: chain_of_next_event,
        });

        return self.unstack_events_to_resolve_this_turn();
    }

    fn unstack_events_to_resolve_this_turn(&mut self) -> ViewModel {
        let mut lines = vec![];

        while !!!self.events_to_resolve_this_turn.is_empty() {
            let mut event_chain_to_play = self.events_to_resolve_this_turn.pop().unwrap();
            if event_chain_to_play.timer != 0 {
                // The event is scheduled for later
                event_chain_to_play.timer = event_chain_to_play.timer - 1;
                self.events_to_resolve_later.push(event_chain_to_play);
                continue;
            }

            lines.push(event_chain_to_play.event.text.clone());
            if event_chain_to_play.event.choices.is_some() {
                // We encountered a choice the player has to make
                self.events_to_resolve_this_turn
                    .push(event_chain_to_play.clone());
                return ViewModel {
                    lines,
                    choices: event_chain_to_play
                        .event
                        .choices
                        .unwrap()
                        .iter()
                        .map(|choice| choice.clone().text)
                        .collect(),
                };
            }

            event_chain_to_play.event.apply_effects(
                &mut self.state,
                &self
                    .event_store
                    .event_chains
                    .iter()
                    .find(|x| x.title == event_chain_to_play.event_chain_id)
                    .unwrap()
                    .effects,
            );

            if event_chain_to_play.event.next.is_some() {
                let next = self.select_next_event(&event_chain_to_play.event.next.unwrap());
                let next_event = self
                    .event_store
                    .clone()
                    .get_event(next.event.clone())
                    .unwrap();
                let chain_of_next_event = self
                    .event_store
                    .clone()
                    .get_chain(next.clone().event)
                    .unwrap();
                self.events_to_resolve_this_turn.push(OngoingEventChain {
                    timer: next.timer.unwrap_or_default(),
                    event: next_event,
                    event_chain_id: chain_of_next_event,
                });
            }
        }

        // We resolved every event that could be during this turn;
        return ViewModel {
            lines,
            choices: vec![],
        };
    }

    fn has_lost(&self) -> bool {
        return self.state.money <= 0 || self.state.population == 0;
    }

    fn has_won(&self) -> bool {
        self.state.population == 8
    }

    fn select_event_chains(&self) -> Vec<OngoingEventChain> {
        return self
            .event_store
            .event_chains
            .iter()
            .filter(|chain| {
                chain.trigger.is_none() || chain.trigger.as_ref().unwrap().is_satisfied(&self.state)
            })
            .filter(|chain| {
                !!!self
                    .events_to_resolve_later
                    .iter()
                    .find(|e| e.event_chain_id == chain.title)
                    .is_some()
            })
            .filter(|_chain| self.random_generator.generate(0, 100) < 80)
            .map(|chain| OngoingEventChain {
                timer: 0,
                event_chain_id: chain.title.clone(),
                event: chain.events.get("start").unwrap().clone(),
            })
            .collect::<Vec<OngoingEventChain>>();
    }

    fn select_choice_outcome(&self, outcomes: &Vec<ChoiceOutcome>) -> ChoiceOutcome {
        let total_weights = outcomes
            .iter()
            .map(|outcome| outcome.weight.unwrap_or(1))
            .reduce(|total: u32, weight: u32| total + weight)
            .unwrap();

        let mut random = self.random_generator.generate(0, total_weights);

        for outcome in outcomes {
            let w = outcome.weight.unwrap_or(1);
            if random < w {
                return outcome.clone();
            }
            random = random - w;
        }

        return outcomes.first().unwrap().clone(); // should never happen
    }

    fn select_next_event(&self, nexts: &Vec<Next>) -> Next {
        let total_weights = nexts
            .iter()
            .map(|next| next.weight.unwrap_or(1))
            .reduce(|total: u32, weight: u32| total + weight)
            .unwrap();

        let mut random = self.random_generator.generate(0, total_weights);

        for next in nexts {
            let w = next.weight.unwrap_or(1);
            if random < w {
                return next.clone();
            }
            random = random - w;
        }

        return nexts.first().unwrap().clone(); // should never happen
    }

    pub fn get_state(&self) -> &State {
        return &self.state;
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::{
        engine::Engine,
        event::Next,
        random::{PseudoRandomGenerator, TestRandomGenerator},
    };

    #[test]
    fn select_next_event_should_return_unique_outcome() {
        let engine = Engine::new("", PseudoRandomGenerator {});

        let next = engine.select_next_event(&vec![Next {
            event: "unique_outcome".to_string(),
            timer: None,
            weight: Some(4),
        }]);
        assert_eq!(
            next,
            Next {
                event: "unique_outcome".to_string(),
                timer: None,
                weight: Some(4),
            }
        );
    }

    #[test]
    fn select_next_event_should_return_a_random_outcome_respecting_weight() {
        let engine = Engine::new(
            "",
            TestRandomGenerator {
                expected_min: 0,
                expected_max: 4,
                return_value: 2,
            },
        );

        let next = engine.select_next_event(&vec![
            Next {
                event: "first_outcome".to_string(),
                timer: None,
                weight: Some(1),
            },
            Next {
                event: "second_outcome".to_string(),
                timer: None,
                weight: Some(1),
            },
            Next {
                event: "third_outcome".to_string(),
                timer: None,
                weight: Some(2),
            },
        ]);

        assert_eq!(
            next,
            Next {
                event: "third_outcome".to_string(),
                timer: None,
                weight: Some(2),
            }
        );
    }

    #[test]
    fn select_next_event_should_use_1_as_the_default_weight() {
        let engine = Engine::new(
            "",
            TestRandomGenerator {
                expected_min: 0,
                expected_max: 2,
                return_value: 0,
            },
        );

        let next = engine.select_next_event(&vec![
            Next {
                event: "first_outcome".to_string(),
                timer: None,
                weight: None,
            },
            Next {
                event: "second_outcome".to_string(),
                timer: None,
                weight: None,
            },
        ]);

        assert_eq!(
            next,
            Next {
                event: "first_outcome".to_string(),
                timer: None,
                weight: None,
            }
        );
    }
}
