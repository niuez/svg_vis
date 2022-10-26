use svg::node::Value;

pub struct Percentage(f32);

macro_rules! implement {
    ($($primitive:ty,)*) => (
        $(impl From<$primitive> for Percentage {
            #[inline]
            fn from(inner: $primitive) -> Self {
                assert!(0.0 <= inner as f32 && inner as f32 <= 1.0);
                Percentage(inner as f32)
            }
        })*
    );
}

implement! {
    i8, i16, i32, i64, isize,
    u8, u16, u32, u64, usize,
    f32, f64,
}

impl From<Percentage> for Value {
    #[inline]
    fn from(inner: Percentage) -> Self {
        inner.0.into()
    }
}
