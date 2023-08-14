use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::engine::{
    chain_store::ChainStore,
    effect::{Effect, EffectType},
    event::{ChoiceOutcome, Event, Next},
    random::RandomGenerator,
    state::State,
};

#[derive(Clone, Debug)]
pub enum ViewModel {
    InGame(InGameView),
    EndOfGame(EndOfGameView),
}

#[derive(Clone, Debug)]
pub struct InGameView {
    pub(crate) lines: Vec<String>,
    pub(crate) choices: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct EndOfGameView {
    pub(crate) is_victory: bool,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OngoingEventChain {
    timer: u32,
    pub(crate) event: Event,
    pub(crate) chain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Engine<Rng> {
    turn: u32,
    state: State,
    chain_store: ChainStore,
    events_to_resolve_this_turn: Vec<OngoingEventChain>,
    events_to_resolve_later: Vec<OngoingEventChain>,
    ongoing_permanent_effects: HashSet<Effect>,
    just_applied_permanent_effects: HashSet<Effect>,
    random_generator: Rng,
    cooling_down_chains: HashMap<String, u32>
}

impl<Rng: RandomGenerator> Engine<Rng> {
    pub fn new(chains_files: Vec<String>, random_generator: Rng) -> Engine<Rng> {
        return Engine {
            turn: 1,
            state: State::new(1, 12, 1000),
            chain_store: ChainStore::new(chains_files),
            events_to_resolve_this_turn: Default::default(),
            events_to_resolve_later: Default::default(),
            ongoing_permanent_effects: Default::default(),
            just_applied_permanent_effects: Default::default(),
            random_generator,
            cooling_down_chains: Default::default(),
        };
    }

    pub fn next_cycle(&mut self) -> ViewModel {
        if !self.events_to_resolve_this_turn.is_empty() {
            panic!("next_cycle called when some events of current turn were waiting to be resolved");
        }

        // Apply chain cooldowns
        for (chain, timer) in self.cooling_down_chains.clone() {
            if timer == 0 {
                self.cooling_down_chains.remove(&chain);
            } else {
                self.cooling_down_chains.insert(chain, timer - 1);
            }
        }


        // Apply "natural" effects
        self.state.evolve();

        // Apply ongoing effects
        let ongoing_effects_to_apply = self.ongoing_permanent_effects.iter()
            .filter(|effect| !self.just_applied_permanent_effects.contains(effect));
        ongoing_effects_to_apply.for_each(|effect| effect.apply(&mut self.state));
        self.just_applied_permanent_effects.clear();

        // Queue new chains
        let mut new_chains = self.select_chains();
        while !!!new_chains.is_empty() {
            self.events_to_resolve_this_turn
                .push(new_chains.pop().unwrap());
        }

        // Queue ongoing chains
        while !!!self.events_to_resolve_later.is_empty() {
            self.events_to_resolve_this_turn
                .push(self.events_to_resolve_later.pop().unwrap());
        }

        return self.unstack_events_to_resolve_this_turn();
    }

    pub fn make_choice(&mut self, index: usize) -> ViewModel {
        if self.events_to_resolve_this_turn.is_empty() {
            panic!("make_choice called when no events of current turn were waiting to be resolved");
        }

        self.just_applied_permanent_effects.clear();

        let event_with_choice = self.events_to_resolve_this_turn.pop().unwrap().event;
        let binding = event_with_choice.choices.unwrap();
        let next = binding.get(index).unwrap();

        let outcome = self.select_choice_outcome(&next.next);

        let next_event = self
            .chain_store
            .clone()
            .get_event(outcome.clone().event)
            .expect(format!("Could not find outcome event '{}' in chain_store, this means the chain definition is invalid", outcome.clone().event).as_str());

        let chain_of_next_event = self.chain_store.clone().get_containing_event(outcome.event).unwrap().title;
        self.events_to_resolve_this_turn.push(OngoingEventChain {
            timer: outcome.timer.unwrap_or_default(),
            event: next_event,
            chain: chain_of_next_event,
        });

        return self.unstack_events_to_resolve_this_turn();
    }

    fn unstack_events_to_resolve_this_turn(&mut self) -> ViewModel {
        let mut lines = vec![];

        while !!!self.events_to_resolve_this_turn.is_empty() {
            let mut event_to_play = self.events_to_resolve_this_turn.pop().unwrap();
            if event_to_play.timer != 0 {
                // The event is scheduled for later
                event_to_play.timer = event_to_play.timer - 1;
                self.events_to_resolve_later.push(event_to_play);
                continue;
            }

            if !event_to_play.event.text.is_empty() {
                lines.push(event_to_play.event.text.clone());
            }

            // Apply effects, if any
            if event_to_play.event.effects.is_some() {
                self.apply_effects(&event_to_play.event.effects.clone().unwrap(), &event_to_play.chain);
            }

            if event_to_play.event.choices.is_some() {
                // We encountered a choice the player has to make
                self.events_to_resolve_this_turn
                    .push(event_to_play.clone());

                // Test for win and lose conditions
                if self.has_won() {
                    return ViewModel::EndOfGame {
                        0: EndOfGameView { is_victory: true }
                    };
                }
                if self.has_lost() {
                    return ViewModel::EndOfGame {
                        0: EndOfGameView { is_victory: false }
                    };
                }

                return ViewModel::InGame {
                    0: InGameView {
                        lines,
                        choices: event_to_play
                            .event
                            .choices
                            .unwrap()
                            .iter()
                            .map(|choice| choice.clone().text)
                            .collect(),
                    }
                };
            }

            if event_to_play.event.next.is_some() {
                let next = self.select_next_event(&event_to_play.event.next.unwrap());
                let next_event = self
                    .chain_store
                    .clone()
                    .get_event(next.event.clone())
                    .expect(format!("Could not find event '{}' in chain_store, this means the chain definition is invalid", next.event.clone()).as_str());
                let chain_of_next_event = self
                    .chain_store
                    .clone()
                    .get_containing_event(next.clone().event)
                    .unwrap()
                    .title;
                self.events_to_resolve_this_turn.push(OngoingEventChain {
                    timer: next.timer.unwrap_or_default(),
                    event: next_event,
                    chain: chain_of_next_event,
                });
            } else {
                let chain = self.chain_store.clone().get_by_name(event_to_play.chain).unwrap();
                self.cooling_down_chains.insert(chain.title, chain.cooldown);
            }
        }

        // We resolved every event that could be during this turn
        // Test for win and lose conditions
        if self.has_won() {
            return ViewModel::EndOfGame {
                0: EndOfGameView { is_victory: true }
            };
        }
        if self.has_lost() {
            return ViewModel::EndOfGame {
                0: EndOfGameView { is_victory: false }
            };
        }

        return ViewModel::InGame {
            0: InGameView {
                lines,
                choices: vec![],
            }
        };
    }

    fn has_lost(&self) -> bool {
        return self.state.money <= 0 || self.state.population == 0;
    }

    fn has_won(&self) -> bool {
        self.state.population == 8
    }

    fn select_chains(&self) -> Vec<OngoingEventChain> {
        return self
            .chain_store
            .chains
            .iter()
            .filter(|chain| {
                chain.trigger.is_none() || chain.trigger.as_ref().unwrap().is_satisfied(&self.state)
            })
            .filter(|chain| {
                !!!self.cooling_down_chains.contains_key(&chain.title)
            })
            .filter(|chain| {
                !!!self
                    .events_to_resolve_later
                    .iter()
                    .find(|e| e.chain == chain.title)
                    .is_some()
            })
            .filter(|_chain| self.random_generator.generate(0, 100) < 80)
            .map(|chain| OngoingEventChain {
                timer: 0,
                chain: chain.title.clone(),
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

    fn apply_effects(&mut self, effects: &HashMap<String, bool>, chain_id: &String) {
        let effect_activations = effects.iter()
            .filter(|(_, activate)| **activate)
            .map(|(effect_name, _)| effect_name)
            .map(|effect| self.chain_store.clone().get_by_name(chain_id.clone()).unwrap().effects.get(effect).unwrap().to_owned());

        let activated_instant_effects = effect_activations
            .clone()
            .filter(|effect| effect.effect_type == EffectType::Instant);
        activated_instant_effects.for_each(|effect| effect.apply(&mut self.state));

        let cloned_ongoing_permanent_effects = self.ongoing_permanent_effects.clone();

        let newly_activated_permanent_effects = effect_activations
            .clone()
            .filter(|effect| effect.effect_type == EffectType::Permanent)
            .filter(|permanent_effect: &Effect| !cloned_ongoing_permanent_effects.contains(&permanent_effect));

        for effect in newly_activated_permanent_effects {
            effect.apply(&mut self.state);
            self.ongoing_permanent_effects.insert(effect.clone());
            self.just_applied_permanent_effects.insert(effect);
        }

        let effect_deactivations = effects.iter()
            .filter(|(_, activate)| !**activate)
            .map(|(effect_name, _)| effect_name)
            .map(|effect| self.chain_store.clone().get_by_name(chain_id.clone()).unwrap().effects.get(effect).unwrap().to_owned());
        effect_deactivations.for_each(|effect| {
            self.ongoing_permanent_effects.remove(&effect);
        });
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
        let engine = Engine::new(vec![], PseudoRandomGenerator {});

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
            vec![],
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
            vec![],
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
