use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::event::{Event, ChoiceOutcome, Next};
use crate::event_store::EventStore;
use crate::log;
use crate::state::State;
use crate::ui_orchestrator::ViewModel;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OngoingEventChain {
    timer: u32,
    pub(crate) event: Event,
    pub(crate) event_chain_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Engine {
    turn: u32,
    state: State,
    event_store: EventStore,
    events_to_resolve_this_turn: Vec<OngoingEventChain>,
    events_to_resolve_later: Vec<OngoingEventChain>,
}

impl Engine {
    pub fn new() -> Engine {
        return Engine {
            turn: 1,
            state: State::new(1, 12, 1000),
            event_store: EventStore::new(),
            events_to_resolve_this_turn: Default::default(),
            events_to_resolve_later: Default::default(),
        };
    }

    pub fn next_cycle(&mut self) -> ViewModel {
        // TODO: also apply events
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
            self.events_to_resolve_this_turn.push(new_event_chains.pop().unwrap())
        }

        // Queue ongoing event chains
        while !!!self.events_to_resolve_later.is_empty() {
            self.events_to_resolve_this_turn.push(self.events_to_resolve_later.pop().unwrap())
        }

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
                self.events_to_resolve_this_turn.push(event_chain_to_play.clone());
                return ViewModel {
                    lines,
                    choices: event_chain_to_play.event.choices.unwrap().iter().map(|choice| choice.clone().text).collect(),
                }
            }

            if event_chain_to_play.event.next.is_some() {
                let next = Engine::select_next_event(&event_chain_to_play.event.next.unwrap());
                let next_event = self.event_store.clone().get_event(next.event.clone()).unwrap();
                let chain_of_next_event = self.event_store.clone().get_chain(next.event).unwrap();
                self.events_to_resolve_this_turn.push(OngoingEventChain {
                    timer: next.timer.unwrap_or_default(),
                    event: next_event,
                    event_chain_id: chain_of_next_event,
            });
            }
        }

        // We resolved every event that could be during this turn;
        return ViewModel { lines, choices: vec![] };
    }

    pub fn make_choice(&mut self, index: usize) -> ViewModel {
        // TODO: also apply events
        if self.events_to_resolve_this_turn.is_empty() {
            panic!("make_choice called when no events of current turn were waiting to be resolved")
        }

        let event_with_choice = self.events_to_resolve_this_turn.pop().unwrap().event;
        let binding = event_with_choice.choices
            .unwrap();
        let next = binding
            .get(index)
            .unwrap();

        let outcome = Engine::select_choice_outcome(&next.next);

        let next_event = self.event_store.clone().get_event(outcome.clone().event).unwrap();
        let chain_of_next_event = self.event_store.clone().get_chain(outcome.event).unwrap();
        self.events_to_resolve_this_turn.push(OngoingEventChain {
            timer: outcome.timer.unwrap_or_default(),
            event: next_event,
            event_chain_id: chain_of_next_event,
        });

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
                self.events_to_resolve_this_turn.push(event_chain_to_play.clone());
                return ViewModel {
                    lines,
                    choices: event_chain_to_play.event.choices.unwrap().iter().map(|choice| choice.clone().text).collect(),
                }
            }

            if event_chain_to_play.event.next.is_some() {
                let next = Engine::select_next_event(&event_chain_to_play.event.next.unwrap());
                let next_event = self.event_store.clone().get_event(next.event.clone()).unwrap();
                let chain_of_next_event = self.event_store.clone().get_chain(next.clone().event).unwrap();
                self.events_to_resolve_this_turn.push(OngoingEventChain {
                    timer: next.timer.unwrap_or_default(),
                    event: next_event,
                    event_chain_id: chain_of_next_event,
            });
            }
        }

        // We resolved every event that could be during this turn;
        return ViewModel { lines, choices: vec![] };
    }

    fn has_lost(&self) -> bool {
        return self.state.money <= 0 ||
            self.state.population == 0;
    }

    fn has_won(&self) -> bool {
        self.state.population == 8
    }

    fn select_event_chains(&self) -> Vec<OngoingEventChain> {
        return self
            .event_store
            .event_chains
            .iter()
            .filter(|chain| chain.trigger.is_none() || chain.trigger.as_ref().unwrap().is_satisfied(&self.state))
            .filter(|chain| !!!self.events_to_resolve_later.iter()
                .find(|e| e.event_chain_id == chain.title)
                .is_some()
            )
            .filter(|_chain| rand::thread_rng().gen_range(0..100) < 80)
            .map(|chain| OngoingEventChain {
                timer: 0,
                event_chain_id: chain.title.clone(),
                event: chain.events.get("start").unwrap().clone(),
            })
            .collect::<Vec<OngoingEventChain>>();
    }

    fn select_choice_outcome(outcomes: &Vec<ChoiceOutcome>) -> ChoiceOutcome {
        let total_weights = outcomes.iter()
            .map(|outcome| outcome.weight.unwrap_or(1))
            .reduce(|total: u32, weight: u32| total + weight)
            .unwrap();

        let mut random = rand::thread_rng().gen_range(0..total_weights);

        for outcome in outcomes {
            let w = outcome.weight.unwrap_or(1);
            if random < w {
                return outcome.clone();
            }
            random = random - w;
        }

        return outcomes.first().unwrap().clone(); // should never happen
    }

    fn select_next_event(nexts: &Vec<Next>) -> Next {
        let total_weights = nexts.iter()
            .map(|next| next.weight.unwrap_or(1))
            .reduce(|total: u32, weight: u32| total + weight)
            .unwrap();

        let mut random = rand::thread_rng().gen_range(0..total_weights);

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
