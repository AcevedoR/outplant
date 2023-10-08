#[derive(Debug)]
pub enum Language {
    LocaleEnUs,
    LocaleFrFr,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::LocaleEnUs => "en_US",
            Language::LocaleFrFr => "fr_FR",
        }
    }
}
