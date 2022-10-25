use svg::node::{
    Node
};
use crate::literal::{
    Length,
};

pub trait AbsPos {
    type Output: Node;
    fn set_abs_pos<X: Into<Length>, Y: Into<Length>>(self, x: X, y: Y) -> Self::Output;
}
