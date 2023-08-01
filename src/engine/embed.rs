use std::error::Error;
#[allow(unused_imports)]
use std::fs;

use rust_embed::{EmbeddedFile, RustEmbed};
use serde_json;
use simple_error::SimpleError;

use crate::engine::event_chain::EventChain;

#[derive(RustEmbed)]
#[folder = "event_chains/"]
#[exclude = "schema.json"]
struct EmbedEventChains;

#[cfg(not(feature = "integration-test"))]
pub fn get_event_chains(_event_chains_filepath: &str) -> Vec<EventChain> {
    get_event_embedded_chains()
}

#[cfg(feature = "integration-test")]
pub fn get_event_chains(event_chains_filepath: &str) -> Vec<EventChain> {
    if event_chains_filepath == "" {
        return vec![];
    }
    let file = fs::read_to_string(event_chains_filepath);
    let event_chain: EventChain = serde_json::from_str(&*file.unwrap()).unwrap();
    return vec![event_chain];
}

#[allow(dead_code)]
fn get_event_embedded_chains() -> Vec<EventChain> {
    let mut chains: Vec<EventChain> = Vec::new();
    for file in EmbedEventChains::iter() {
        chains.push(read_event_chain(file.into_owned()).unwrap());
    }

    return chains;
}

fn read_event_chain(file_path: String) -> Result<EventChain, Box<dyn Error>> {
    let binding = EmbedEventChains::get(&file_path)
        .ok_or_else(|| SimpleError::new("failed to retrieve json file"));
    let json_chain = binding
        .as_ref()
        .and_then(|c: &EmbeddedFile| Ok(c.data.as_ref()))
        .unwrap();

    let chain: EventChain = serde_json::from_slice(json_chain)?;

    Ok(chain)
}
