use std::borrow::Cow;
use std::fs;
use std::{collections::HashMap, error::Error, str::from_utf8};

use rust_embed::RustEmbed;
use simple_error::SimpleError;

#[derive(RustEmbed)]
#[folder = "chains/"]
#[include = "*.tsv"]
struct EmbeddedTranslations;

pub fn get_translations(
    translation_override_directory_path_opt: Option<&str>,
) -> HashMap<String, HashMap<String, String>> {
    if let Some(translation_override_directory_path) = translation_override_directory_path_opt {
        return get_translations_from_directory(translation_override_directory_path);
    }

    let mut translations: HashMap<String, HashMap<String, String>> = HashMap::new();

    for embedded_file in EmbeddedTranslations::iter() {
        parse_and_add_translations(
            &mut translations,
            embedded_file.clone(),
            extract_embedded_translation(embedded_file.to_string())
                .expect("failed to read embedded translation file"),
        );
    }
    return translations;
}

// this is in reality only used for integration tests
pub fn get_translations_from_directory(
    translation_override_directory_path: &str,
) -> HashMap<String, HashMap<String, String>> {
    let mut translations: HashMap<String, HashMap<String, String>> = HashMap::new();
    let test_translation_directory = fs::read_dir(translation_override_directory_path)
        .expect("unable to read test translation dir");

    for translation_file in test_translation_directory {
        let buf = translation_file.unwrap().path();
        let file_path = buf.file_name();
        if file_path.is_some() && file_path.unwrap().to_string_lossy().contains(".tsv") {
            let file_content = fs::read_to_string(buf.clone());
            parse_and_add_translations(
                &mut translations,
                file_path.unwrap().to_string_lossy(),
                file_content.expect("failed to read translation file override"),
            )
        }
    }
    return translations;
}

fn parse_and_add_translations(
    translations: &mut HashMap<String, HashMap<String, String>>,
    file: Cow<str>,
    raw_translations: String,
) {
    parse_translation_file(raw_translations)
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

fn extract_embedded_translation(file_path: String) -> Result<String, Box<dyn Error>> {
    let binding = EmbeddedTranslations::get(&file_path)
        .ok_or_else(|| SimpleError::new("failed to retrieve translation file"));

    let raw_translations = binding
        .as_ref()
        .and_then(|c| Ok(c.data.as_ref()))
        .map(|data| from_utf8(data).unwrap())
        .unwrap();

    return Ok(raw_translations.to_string());
}

fn parse_translation_file(raw_translations: String) -> HashMap<String, String> {
    let mut translations: HashMap<String, String> = HashMap::new();
    for raw_translation in raw_translations.lines() {
        let s: Vec<&str> = raw_translation.split("\t").collect();
        translations.insert(s[0].to_string(), s[1].to_string());
    }
    return translations;
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
