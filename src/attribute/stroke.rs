use crate::literal::{
    Color,
    Percentage,
};

pub trait Stroke {
    fn stroke<C: Into<Color>>(self, color: C) -> Self;
    fn stroke_opacity<P: Into<Percentage>>(self, per: P) -> Self;
}

macro_rules! implement_stroke {
    ($T:ty, $svg:ident) => (
        impl Stroke for $T {
            fn stroke<C: Into<Color>>(mut self, color: C) -> Self {
                self.$svg = self.$svg.set("stroke", color.into());
                self
            }
            fn stroke_opacity<P: Into<Percentage>>(mut self, per: P) -> Self {
                self.$svg = self.$svg.set("stroke-opacity", per.into());
                self
            }
        }
    );
}

pub(crate) use implement_stroke;
