use svg::node::Value;
use svg::node::element::Circle as SvgCircle;
use crate::attribute::{
    AbsPos,
    Fill,
    Stroke,
    StrokeWidth,
};
use crate::literal::{
    Length,
    Color,
};


pub struct Circle {
    svg: SvgCircle,
}

impl Circle {
    pub fn new() -> Self {
        Circle {
            svg: SvgCircle::new()
        }
    }
    pub fn radius<V: Into<Value>>(mut self, r: V) -> Self {
        self.svg = self.svg.set("r", r);
        self
    }
}

impl AbsPos for Circle {
    type Output = SvgCircle;
    fn set_abs_pos<X: Into<Length>, Y: Into<Length>>(self, x: X, y: Y) -> Self::Output {
        self.svg
            .set("cx", x.into())
            .set("cy", y.into())
    }
}

crate::attribute::fill::implement_fill! { Circle, svg }
crate::attribute::stroke::implement_stroke! { Circle, svg }
crate::attribute::stroke_width::implement_stroke_width! { Circle, svg }

/*
impl Fill for Circle {
    fn fill<C: Into<Color>>(mut self, color: C) -> Self {
        self.svg = self.svg.set("fill", color.into());
        self
    }
}
*/

#[cfg(test)]
mod circle_tests {
    use crate::chart::Chart;
    use super::Circle;
    use crate::attribute::Fill;
    #[test]
    fn circle_test1() {
        let ch = Chart::new(0, 0, 100, 100);
        let c1 = Circle::new().radius(5).fill("red");
        let c2 = Circle::new().radius(10).fill("blue");
        let c3 = Circle::new().radius(15).fill("green");
        let c4 = Circle::new().radius(20).fill("black");
        let ch = ch
            .draw(c1, 5, 50)
            .draw(c2, 20, 50)
            .draw(c3, 45, 50)
            .draw(c4, 80, 50);
        println!("{}", ch);
    }
}
