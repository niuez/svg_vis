use svg::node::Value;
use svg::node::element::Circle as SvgCircle;
use crate::attribute::{
    AbsPos
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
    fn set_abs_pos<X: Into<Value>, Y: Into<Value>>(self, x: X, y: Y) -> Self::Output {
        self.svg
            .set("cx", x)
            .set("cy", y)
    }
}

#[cfg(test)]
mod circle_tests {
    use crate::chart::Chart;
    use super::Circle;
    #[test]
    fn circle_test1() {
        let ch = Chart::new(0, 0, 100, 100);
        let c1 = Circle::new().radius(5);
        let c2 = Circle::new().radius(10);
        let c3 = Circle::new().radius(15);
        let c4 = Circle::new().radius(20);
        let ch = ch
            .draw(c1, 5, 50)
            .draw(c2, 20, 50)
            .draw(c3, 45, 50)
            .draw(c4, 80, 50);
        println!("{}", ch);
    }
}
