use std::error::Error;

use crate::event_chain::EventChain;
use rust_embed::{EmbeddedFile, RustEmbed};
use serde_json;

use simple_error::SimpleError;

#[derive(RustEmbed)]
#[folder = "event_chains/"]
#[exclude = "schema.json"]
struct EmbedEventChains;

pub fn get_event_chains() -> Vec<EventChain> {
    let mut chains: Vec<EventChain> = Vec::new();
    for file in EmbedEventChains::iter() {
        chains.push(read_event_chain(file.into_owned()).unwrap());
    }

    println!("EEEEEEEEEEEEEEEEEEE");
    println!("{}", chains.len());

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
