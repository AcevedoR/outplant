use std::error::Error;
#[allow(unused_imports)]
use std::fs;
use std::collections::HashMap;

use rust_embed::{EmbeddedFile, RustEmbed};
use serde::{Deserialize, Serialize};
use serde_json;
use simple_error::SimpleError;

use crate::engine::chain::Chain;

use super::{trigger::Trigger, event::Event, effect::Effect};

#[derive(RustEmbed)]
#[folder = "chains/"]
#[exclude = "schema.json"]
#[exclude = "*.tsv"]
struct EmbeddedChains;

pub fn get_chains(chain_files_override_paths: Vec<String>) -> Vec<JsonChain> {
    let mut chains: Vec<JsonChain> = Vec::new();

    if chain_files_override_paths.is_empty() {
        for file in EmbeddedChains::iter() {
            chains.push(read_embedded_chain(file.into_owned()).unwrap());
        }
    } else {
        // this is in reality only used for integration tests
        for chain_file in chain_files_override_paths.iter() {
            let file = fs::read_to_string(chain_file);
            let mut json_chain: JsonChain = serde_json::from_str(&*file.unwrap()).unwrap();
            chains.push(json_chain);
        }
    }

    return chains;
}

fn read_embedded_chain(file_path: String) -> Result<JsonChain, Box<dyn Error>> {
    let binding = EmbeddedChains::get(&file_path)
        .ok_or_else(|| SimpleError::new("failed to retrieve json file"));
    let raw_chain = binding
        .as_ref()
        .and_then(|c: &EmbeddedFile| Ok(c.data.as_ref()))
        .unwrap();

    let mut json_chain: JsonChain = serde_json::from_slice(raw_chain)?;

    Ok(json_chain)
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonChain {
    pub title: String,
    pub cooldown: u32,
    pub trigger: Option<Trigger>,
    pub events: HashMap<String, Event>,
    pub effects: HashMap<String, Effect>,
    pub auto_select: Option<bool>,
}



impl JsonChain {
    pub fn to_chain(&mut self) -> Chain {
        Chain{
            title: self.title.clone(),
            cooldown: self.cooldown,
            trigger: self.trigger.clone(),
            auto_select: self.auto_select.is_some_and(|auto_select| auto_select),
        }
    }
}
