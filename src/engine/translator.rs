use std::collections::HashMap;

use crate::engine::embed_translations::get_translations;

#[derive(Clone, Debug)]
pub struct Translator {
    translations: HashMap<String, HashMap<String, String>>,
}

impl Translator {
    pub fn new(translation_override_directory_path: Option<&str>) -> Translator {
        return Translator {
            translations: get_translations(translation_override_directory_path),
        };
    }

    pub fn translate(self, line: String, locale: &str) -> String {
        match extract_tag(line.clone()) {
            Some(tag) => self
                .translations
                .get(locale)
                .and_then(|locale_translations| locale_translations.get(&tag))
                .map(|text| text.to_string())
                .unwrap_or(remove_tag(line)),
            None => line,
        }
    }
}

fn extract_tag(line: String) -> Option<String> {
    if line.len() < 7 {
        return None;
    }

    if line.chars().nth(line.chars().count() - 7).unwrap() == '#' {
        return Some((&line.to_string())[line.len() - 6..].to_string());
    } else {
        return None;
    }
}

fn remove_tag(line: String) -> String {
    line[..line.len() - 7].trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use crate::engine::translator::extract_tag;
    use crate::engine::translator::remove_tag;

    #[test]
    fn extract_tag_true() {
        assert_eq!(
            extract_tag("That line has a tag. #12aaaa".to_string()),
            Some("12aaaa".to_string()),
        )
    }

    #[test]
    fn extract_tag_false() {
        assert_eq!(extract_tag("That line has no tag.".to_string()), None,)
    }

    #[test]
    fn extract_tag_short() {
        assert_eq!(extract_tag("Short".to_string()), None,)
    }

    #[test]
    fn extract_tag_unicode() {
        assert_eq!(
            extract_tag("“Oh, there is a discount on the MIL-SPEC model! This module is even invisible to most sensors and emits customizable noise-canceling waves!“ [300 €€€] #y9zg0n".to_string()),
            Some("y9zg0n".to_string()),
        )
    }

    #[test]
    fn remove_tag_should_remove_tag() {
        assert_eq!(
            remove_tag("That line has a tag. #12aaaa".to_string()),
            "That line has a tag.".to_string(),
        )
    }
}
