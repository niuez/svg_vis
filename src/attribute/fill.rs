use crate::literal::{
    Color,
    Percentage,
};

pub trait Fill {
    fn fill<C: Into<Color>>(self, color: C) -> Self;
    fn fill_opacity<P: Into<Percentage>>(self, per: P) -> Self;
}

macro_rules! implement_fill {
    ($T:ty, $svg:ident) => (
        impl Fill for $T {
            fn fill<C: Into<Color>>(mut self, color: C) -> Self {
                self.$svg = self.$svg.set("fill", color.into());
                self
            }
            fn fill_opacity<P: Into<Percentage>>(mut self, per: P) -> Self {
                self.$svg = self.$svg.set("fill-opacity", per.into());
                self
            }
        }
    );
}

pub(crate) use implement_fill;
