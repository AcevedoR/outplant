use std::error::Error;
#[allow(unused_imports)]
use std::fs;

use rust_embed::{EmbeddedFile, RustEmbed};
use serde_json;
use simple_error::SimpleError;

use crate::engine::chain::Chain;

#[derive(RustEmbed)]
#[folder = "chains/"]
#[exclude = "schema.json"]
struct EmbeddedChains;

pub fn get_chains(chain_files: Vec<String>) -> Vec<Chain> {
    let mut chains: Vec<Chain> = Vec::new();

    if chain_files.is_empty() {
        for file in EmbeddedChains::iter() {
            chains.push(read_embedded_chain(file.into_owned()).unwrap());
        }
    } else {
        for chain_file in chain_files.iter() {
            let file = fs::read_to_string(chain_file);
            let chain: Chain = serde_json::from_str(&*file.unwrap()).unwrap();
            chains.push(chain);
        }
    }

    return chains;
}

fn read_embedded_chain(file_path: String) -> Result<Chain, Box<dyn Error>> {
    let binding = EmbeddedChains::get(&file_path)
        .ok_or_else(|| SimpleError::new("failed to retrieve json file"));
    let json_chain = binding
        .as_ref()
        .and_then(|c: &EmbeddedFile| Ok(c.data.as_ref()))
        .unwrap();

    let chain: Chain = serde_json::from_slice(json_chain)?;

    Ok(chain)
}
