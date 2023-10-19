use serde::{Deserialize, Serialize};

use std::collections::{HashMap};

use crate::engine::{chain::Chain, embed_chains::get_chains, event::Event, effect::Effect};

use super::utils::prefix_all_keys;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChainStore {
    chains: HashMap<String, Chain>,
    events: HashMap<String, Event>,
    effects: HashMap<String, Effect>,
}

impl ChainStore {
    pub fn new(chain_files_override_paths: Vec<String>) -> ChainStore {
        let mut chains: HashMap<String, Chain> = Default::default();
        let mut events: HashMap<String, Event> = Default::default();
        let mut effects: HashMap<String, Effect> = Default::default();

        for mut json_chain in get_chains(chain_files_override_paths) {
            chains.insert(json_chain.title.clone(), json_chain.to_chain());

            let mut chain_events = prefix_all_keys(&mut json_chain.events, &(json_chain.title.clone() + "/"));

            for (_, event) in chain_events.iter_mut() {
                event.set_namespace(json_chain.title.clone())
            }

            events.extend(chain_events);
            effects.extend(prefix_all_keys(&mut json_chain.effects, &(json_chain.title.clone() + "/")));
        }

        return ChainStore {
            chains: chains,
            effects: effects,
            events: events,
        };
    }

    pub fn get_event(self, event_name: String) -> Event {
        return self.events.get(&event_name)
            .expect(("unknown event required: ".to_owned() + &event_name).as_str())
            .clone();
    }

    pub fn get_effect(self, effect_name: String) -> Effect {
        return self.effects.get(&effect_name)
            .expect(("unknown effect required: ".to_owned() + &effect_name).as_str())
            .clone();
    }

    pub fn get_chain(self, chain_name: String) -> Chain {
        return self.chains.get(&chain_name)
            .expect(("unknown chain required: ".to_owned() + &chain_name).as_str())
            .clone();
    }

    pub fn get_chains(&self) -> &HashMap<String, Chain> {
        &self.chains
    }
}

pub fn extract_chain_title(namespaced_id: &str) -> &str {
    namespaced_id.splitn(2, '/').next().unwrap_or("")
}

#[cfg(test)]
mod tests {
    use crate::engine::chain_store::extract_chain_title;

    #[test]
    fn test_extract_chain_title() {
        let input = "earthquake/report_about_destroyed_shelters";
        let result = extract_chain_title(input);
        assert_eq!(result, "earthquake");
    }
}
