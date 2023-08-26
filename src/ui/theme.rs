pub struct ThemeValues {
    pub main_color: String,
    pub main_color_light: String,
    pub main_color_dark: String,
}

pub enum Theme {
    MagnificentCyanTheme,
    AtrociousPurpleTheme,
}

impl Theme {
    pub fn value(&self) -> ThemeValues {
        match *self {
            Theme::MagnificentCyanTheme => ThemeValues {
                main_color: "#00dfff".to_string(),
                main_color_light: "rgb(198, 249, 249)".to_string(),
                main_color_dark:
                    "color-mix(in srgb, var(--background-color) 65%, var(--main-color))".to_string(),
            },
            Theme::AtrociousPurpleTheme => ThemeValues {
                main_color: "#5e548e".to_string(),
                main_color_light: "#e0b1cb".to_string(),
                main_color_dark: "#3c365a".to_string(),
            },
        }
    }
}
