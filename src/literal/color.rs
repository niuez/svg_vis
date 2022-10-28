use svg::node::Value;

pub enum Color {
    NamedColor(String),
    RGB(f32, f32, f32),
}

impl Color {
    pub fn from_name<T: Into<String>>(str: T) -> Color {
        Color::NamedColor(str.into())
    }
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Color {
        Color::RGB(r, g, b)
    }
}

impl<T: Into<String>> From<T> for Color {
    #[inline]
    fn from(str: T) -> Color {
        Color::NamedColor(str.into())
    }
}

impl From<Color> for Value {
    #[inline]
    fn from(color: Color) -> Value {
        match color {
            Color::NamedColor(color) => color.into(),
            Color::RGB(r, g, b) => {
                let r = (r.clamp(0.0, 1.0) * 255.0).floor() as i32;
                let g = (g.clamp(0.0, 1.0) * 255.0).floor() as i32;
                let b = (b.clamp(0.0, 1.0) * 255.0).floor() as i32;
                format!("#{:02X}{:02X}{:02X}", r, g, b).into()
            }
        }
    }
}

#[cfg(test)]
mod color_tests {
    use super::*;
    #[test]
    fn rgb_test() {
        println!("{}", Value::from(Color::from_rgb(0.8, 0.5, 0.03)));
    }
}
