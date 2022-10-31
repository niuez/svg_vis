use svg::node::element::Text as SvgText;
use svg::node::Text as TextContent;
use svg::node::Value;
use crate::attribute::{
    AbsPos,
    Fill,
    Stroke,
    StrokeWidth,
    TextAnchor,
    DominantBaseline,
    FontSize,
    ID,
};
use crate::attribute::abs_pos::Scale;
use crate::attribute::text_anchor::TextAnchorValue;
use crate::attribute::dominant_baseline::DominantBaselineValue;
use crate::literal::{
    Length,
    Color,
    Percentage,
};

pub struct Text {
    svg: SvgText,
    text: String,
    font_size: Option<Length>,
}

impl Text {
    pub fn new() -> Self {
        Text {
            svg: SvgText::new(),
            text: String::new(),
            font_size: None,
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
crate::attribute::text_anchor::implement_text_anchor!{ Text, svg }
crate::attribute::dominant_baseline::implement_dominant_baseline!{ Text, svg }
crate::attribute::id::implement_id! { Text, svg }

impl FontSize for Text {
    fn font_size<L: Into<Length>>(mut self, size: L) -> Self {
        self.font_size = Some(size.into());
        self
    }
}

impl AbsPos for Text {
    type Output = SvgText;
    fn set_abs_pos<X: Into<Length>, Y: Into<Length>>(self, x: X, y: Y, scale: &Scale) -> Self::Output {
        self.svg
            .add(TextContent::new(self.text))
            .set("x", x.into())
            .set("y", y.into())
            .set("font-size", self.font_size.map(|l| (l * scale.y).into()).unwrap_or(Value::from("medium")))
    }
}


#[cfg(test)]
mod text_tests {
    use crate::chart::Chart;
    use super::Text;
    use crate::element::Path;
    use crate::attribute::{
        Fill,
        TextAnchor,
        DominantBaseline,
        Stroke,
    };
    use crate::attribute::text_anchor::TextAnchorValue;
    use crate::attribute::dominant_baseline::DominantBaselineValue;
    #[test]
    fn text_test1() {
        let ch = Chart::new(0, 0, 60, 70);
        let t1 = Text::new()
            .set_text("##")
            .fill("red")
            .text_anchor(TextAnchorValue::Start);
        let t2 = Text::new()
            .set_text("##")
            .fill("blue")
            .text_anchor(TextAnchorValue::Middle);
        let t3 = Text::new()
            .set_text("##")
            .fill("green")
            .text_anchor(TextAnchorValue::End);
        let center_path = Path::new()
            .move_rel(30, 0)
            .line_rel(0, 70)
            .fill("none")
            .stroke("black");
        let ch = ch
            .draw(center_path, 0, 0)
            .draw(t1, 30, 10)
            .draw(t2, 30, 30)
            .draw(t3, 30, 50);
        println!("{}", ch);
    }
    #[test]
    fn text_test2() {
        let ch = Chart::new(0, 0, 30, 30);
        let t1 = Text::new()
            .set_text("##")
            .fill("red")
            .text_anchor(TextAnchorValue::Middle)
            .dominant_baseline(DominantBaselineValue::Central);
        let center_path = Path::new()
            .move_rel(15, 0)
            .line_rel(0, 30)
            .move_rel(-15, -15)
            .line_rel(30, 0)
            .fill("none")
            .stroke("black");
        let ch = ch
            .draw(center_path, 0, 0)
            .draw(t1, 15, 15);
        println!("{}", ch);
    }
}
