use svg::node::{
    Node
};
use crate::literal::{
    Length,
};

pub struct Scale {
    pub(crate) x: Length,
    pub(crate) y: Length,
}

impl Scale {
    pub fn new() -> Self {
        Scale {
            x: 1.0.into(),
            y: 1.0.into(),
        }
    }
    pub fn xy(scale_x: f32, scale_y: f32) -> Self {
        Scale { x: scale_x.into(), y: scale_y.into() }
    }
}

pub trait AbsPos {
    type Output: Node;
    fn set_abs_pos<X: Into<Length>, Y: Into<Length>>(self, x: X, y: Y, scale: &Scale) -> Self::Output;
}
