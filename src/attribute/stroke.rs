use crate::literal::Color;

pub trait Stroke {
    fn stroke<C: Into<Color>>(self, color: C) -> Self;
}

macro_rules! implement_stroke {
    ($T:ty, $svg:ident) => (
        impl Stroke for $T {
            fn stroke<C: Into<Color>>(mut self, color: C) -> Self {
                self.$svg = self.$svg.set("stroke", color.into());
                self
            }
        }
    );
}

pub(crate) use implement_stroke;
