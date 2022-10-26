use svg::node::element::Text as SvgText;
use svg::node::Text as TextContent;
use crate::attribute::{
    AbsPos,
    Fill,
    Stroke,
    StrokeWidth,
};
use crate::literal::{
    Length,
    Color,
    Percentage,
};

pub struct Text {
    svg: SvgText,
    text: String,
}

impl Text {
    pub fn new() -> Self {
        Text {
            svg: SvgText::new(),
            text: String::new(),
        }
    }
    pub fn set_text<S: ToString>(mut self, text: S) -> Self {
        self.text = text.to_string();
        self
    }
}

crate::attribute::fill::implement_fill! { Text, svg }
crate::attribute::stroke::implement_stroke! { Text, svg }
crate::attribute::stroke_width::implement_stroke_width! { Text, svg }

impl AbsPos for Text {
    type Output = SvgText;
    fn set_abs_pos<X: Into<Length>, Y: Into<Length>>(self, x: X, y: Y) -> Self::Output {
        self.svg
            .add(TextContent::new(self.text))
            .set("x", x.into())
            .set("y", y.into())
    }
}


#[cfg(test)]
mod text_tests {
    use crate::chart::Chart;
    use super::Text;
    use crate::attribute::Fill;
    #[test]
    fn text_test1() {
        let ch = Chart::new(0, 0, 100, 100);
        let t1 = Text::new()
            .set_text("##")
            .fill("red");
        let ch = ch
            .draw(t1, 10, 10);
        println!("{}", ch);
    }
}
