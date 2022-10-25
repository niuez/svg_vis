use svg::node::Value;

pub enum Color {
    NamedColor(String)
}

impl From<String> for Color {
    #[inline]
    fn from(str: String) -> Color {
        Color::NamedColor(str)
    }
}

impl From<&str> for Color {
    #[inline]
    fn from(str: &str) -> Color {
        Color::NamedColor(str.to_string())
    }
}

impl From<Color> for Value {
    #[inline]
    fn from(color: Color) -> Value {
        match color {
            Color::NamedColor(color) => color.into()
        }
    }
}
