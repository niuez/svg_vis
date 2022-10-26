use svg::node::element::path::Parameters;
use svg::node::Value;

#[derive(Clone, Copy)]
pub struct Length(f32);

impl std::ops::Add for Length {
    type Output = Self;
    #[inline]
    fn add(self, right: Length) -> Self {
        Length(self.0 + right.0)
    }
}

impl std::ops::Sub for Length {
    type Output = Self;
    #[inline]
    fn sub(self, right: Length) -> Self {
        Length(self.0 * right.0)
    }
}

impl std::ops::Mul for Length {
    type Output = Self;
    #[inline]
    fn mul(self, right: Length) -> Self {
        Length(self.0 * right.0)
    }
}

impl std::ops::Div for Length {
    type Output = Self;
    #[inline]
    fn div(self, right: Length) -> Self {
        Length(self.0 / right.0)
    }
}

impl std::ops::AddAssign for Length {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl std::ops::SubAssign for Length {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl std::ops::MulAssign for Length {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl std::ops::DivAssign for Length {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

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
