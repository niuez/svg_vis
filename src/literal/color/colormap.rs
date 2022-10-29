pub mod jet;
pub use jet::Jet;

use super::Color;

pub trait ColorMap {
    fn cmap(&self, v: f32) -> Color;
}

pub(crate) fn lin_interpolate(v: f32, data: &[(f32, f32, f32)]) -> (f32, f32, f32) {
    let n = data.len();
    let v = v.clamp(0.0, 1.0) * ((n - 1) as f32);
    let i = v.floor() as usize;
    let j = v.ceil() as usize;
    let r = v - (i as f32);

    let x = &data[i];
    let y = &data[j];
    (
     (y.0 - x.0) * r + x.0, 
     (y.1 - x.1) * r + x.1,
     (y.2 - x.2) * r + x.2
    )
}
