use svg::Document;
use svg::node::Value;
use crate::attribute::{
    AbsPos,
};

pub struct Chart {
    svg: Document,
}

impl Chart {
    pub fn new(x: u64, y: u64, width: u64, height: u64) -> Self {
        Chart {
            svg: Document::new()
                .set("ViewBox", (x, y, width, height)),
        }
    }
    pub fn draw<E: AbsPos, X: Into<Value>, Y: Into<Value>>(mut self, elem: E, x: X, y: Y) -> Self {
        self.svg = self.svg.add(elem.set_abs_pos(x, y));
        self
    }
}


impl std::fmt::Display for Chart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.svg)
    }
}
