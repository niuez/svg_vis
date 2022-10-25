use crate::literal::Color;

pub trait Fill {
    fn fill<C: Into<Color>>(self, color: C) -> Self;
}

macro_rules! implement_fill {
    ($T:ty, $svg:ident) => (
        impl Fill for $T {
            fn fill<C: Into<Color>>(mut self, color: C) -> Self {
                self.$svg = self.$svg.set("fill", color.into());
                self
            }
        }
    );
}

pub(crate) use implement_fill;
