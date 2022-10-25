use crate::literal::Length;

pub trait StrokeWidth {
    fn stroke_width<L: Into<Length>>(self, color: L) -> Self;
}

macro_rules! implement_stroke_width {
    ($T:ty, $svg:ident) => (
        impl StrokeWidth for $T {
            fn stroke_width<L: Into<Length>>(mut self, wid: L) -> Self {
                self.$svg = self.$svg.set("stroke-width", wid.into());
                self
            }
        }
    );
}

pub(crate) use implement_stroke_width;
