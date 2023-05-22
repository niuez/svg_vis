use svg::Document;
use svg::node::Value;
use crate::attribute::{
    ID,
    AbsPos,
};
use crate::literal::Length;
use crate::attribute::abs_pos::Scale;


pub struct Chart {
    svg: Document,
    scale: Scale,
}

impl Chart {
    pub fn new(x: i64, y: i64, width: i64, height: i64) -> Self {
        Chart {
            svg: Document::new()
                .set("ViewBox", (x, y, width, height)),
            scale: Scale::new(),
        }
    }
    pub fn scale(mut self, x: f32, y: f32) -> Self {
        self.scale = Scale::xy(x, y);
        self
    }
    pub fn chart_width<V: Into<Value>>(mut self, w: V) -> Self {
        self.svg = self.svg.set("width", w.into());
        self
    }
    pub fn chart_height<V: Into<Value>>(mut self, h: V) -> Self {
        self.svg = self.svg.set("height", h.into());
        self
    }
    pub fn draw<E: AbsPos, X: Into<Length>, Y: Into<Length>>(mut self, elem: E, x: X, y: Y) -> Self {
        self.svg = self.svg.add(elem.set_abs_pos(x.into() * self.scale.x, y.into() * self.scale.y, &self.scale));
        self
    }
}

impl ID for Chart {
    fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.svg = self.svg.set("id", id.into());
        self
    }
}


impl std::fmt::Display for Chart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.svg)
    }
}
