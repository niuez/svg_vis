use crate::literal::Length;

pub trait FontSize {
    fn font_size<L: Into<Length>>(self, size: L) -> Self;
}
