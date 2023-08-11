use serde::{Deserialize, Serialize};

use crate::engine::{embed::get_chains, event::Event, chain::Chain};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChainStore {
    pub chains: Vec<Chain>,
}

impl ChainStore {
    pub fn new(chain_files: Vec<String>) -> ChainStore {
        return ChainStore {
            chains: get_chains(chain_files),
        };
    }

    pub fn get_event(self, event_name: String) -> Option<Event> {
        for chain in self.chains {
            if chain.events.contains_key(&event_name) {
                return Some(chain.events.get(&event_name).unwrap().clone());
            }
        }
        return None;
    }

    pub fn get_containing_event(self, event_name: String) -> Option<Chain> {
        for chain in self.chains {
            if chain.events.contains_key(&event_name) {
                return Some(chain);
            }
        }
        return None;
    }

    pub fn get_by_name(self, chain_name: String) -> Option<Chain> {
        for chain in self.chains {
            if chain.title == chain_name {
                return Some(chain)
            }
        }
        return None
    }
}
