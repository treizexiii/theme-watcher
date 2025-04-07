use std::fmt::{Display, Formatter};

#[derive(Clone, Copy)]
pub enum ThemeStyle {
    DarkTheme,
    LightTheme,
    None,
}

impl Display for ThemeStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeStyle::DarkTheme => write!(f, "Dark"),
            ThemeStyle::LightTheme => write!(f, "Light"),
            ThemeStyle::None => write!(f, "None"),
        }
    }
}

impl ThemeStyle {
    pub fn new(name: &str) -> ThemeStyle {
        if name.contains("dark") {
            return ThemeStyle::DarkTheme;
        }
        ThemeStyle::LightTheme
    }
}

impl PartialEq for ThemeStyle {
    fn eq(&self, other: &Self) -> bool {
        if let ThemeStyle::DarkTheme = self {
            if let ThemeStyle::DarkTheme = other {
                return true;
            }
        } else if let ThemeStyle::LightTheme = self {
            if let ThemeStyle::LightTheme = other {
                return true;
            }
        }
        false
    }
}
