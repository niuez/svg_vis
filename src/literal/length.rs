use svg::node::element::path::Parameters;
use svg::node::Value;

#[derive(Clone, Copy)]
pub struct Length(f32);

macro_rules! implement {
    ($($primitive:ty,)*) => (
        $(impl From<$primitive> for Length {
            #[inline]
            fn from(inner: $primitive) -> Self {
                Length(inner as f32)
            }
        })*
    );
}

implement! {
    i8, i16, i32, i64, isize,
    u8, u16, u32, u64, usize,
    f32, f64,
}

impl From<Length> for Parameters {
    #[inline]
    fn from(inner: Length) -> Self {
        inner.0.into()
    }
}

impl From<Length> for Value {
    #[inline]
    fn from(inner: Length) -> Self {
        inner.0.into()
    }
}
