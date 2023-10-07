use std::{collections::HashMap, error::Error, str::from_utf8};

use rust_embed::RustEmbed;
use simple_error::SimpleError;

#[derive(RustEmbed)]
#[folder = "chains/"]
#[include = "*.tsv"]
struct EmbeddedTranslations;

pub fn get_translations() -> HashMap<String, HashMap<String, String>> {
    let mut translations: HashMap<String, HashMap<String, String>> = HashMap::new();

    for file in EmbeddedTranslations::iter() {
        read_embedded_translation(file.to_string())
            .expect("failed to read embedded translation file")
            .iter()
            .for_each(|(tag, text)| {
                let locale = get_locale(file.to_string());
                if !translations.contains_key(&locale) {
                    translations.insert(locale.clone(), HashMap::new());
                }
                translations
                    .get_mut(&locale)
                    .unwrap()
                    .insert(tag.clone(), text.clone());
            });
    }

    return translations;
}

fn read_embedded_translation(file_path: String) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut translations: HashMap<String, String> = HashMap::new();

    let binding = EmbeddedTranslations::get(&file_path)
        .ok_or_else(|| SimpleError::new("failed to retrieve translation file"));

    let raw_translations = binding
        .as_ref()
        .and_then(|c| Ok(c.data.as_ref()))
        .map(|data| from_utf8(data).unwrap())
        .unwrap();

    for raw_translation in raw_translations.lines() {
        let s: Vec<&str> = raw_translation.split("\t").collect();
        translations.insert(s[0].to_string(), s[1].to_string());
    }

    Ok(translations)
}

fn get_locale(filename: String) -> String {
    filename[filename.len() - 9..filename.len() - 4].to_string()
}

#[cfg(test)]
mod tests {
    use crate::engine::embed_translations::get_locale;

    #[test]
    fn get_locale_should_get_locale() {
        assert_eq!(
            get_locale("./chains/improved_drones_stealth.fr_FR.tsv".to_string()),
            "fr_FR".to_string(),
        )
    }
}
