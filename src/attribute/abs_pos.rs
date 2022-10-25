use svg::node::{
    Value, Node
};

pub trait AbsPos {
    type Output: Node;
    fn set_abs_pos<X: Into<Value>, Y: Into<Value>>(self, x: X, y: Y) -> Self::Output;
}
