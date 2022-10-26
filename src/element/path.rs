use svg::node::element::Path as SvgPath;
use svg::node::element::path::{
    Data as SvgData,
};
use crate::attribute::{
    AbsPos,
    Fill,
    Stroke,
    StrokeWidth,
};
use crate::attribute::abs_pos::Scale;

use crate::literal::{
    Length,
    Color,
    Percentage,
};

#[derive(Clone)]
enum Command {
    MoveTo(Length, Length),
    MoveBy(Length, Length),
    LineTo(Length, Length),
    LineBy(Length, Length),
    QCurTo(Length, Length, Length, Length),
    QCurBy(Length, Length, Length, Length),
    CCurTo(Length, Length, Length, Length, Length, Length),
    CCurBy(Length, Length, Length, Length, Length, Length),
    Close,
}

#[derive(Clone)]
pub struct Path {
    path: SvgPath,
    data: Vec<Command>,
}

impl Path {
    pub fn new() -> Self {
        Path {
            path: SvgPath::new(),
            data: Vec::new(),
        }
    }

    pub fn move_abs<X: Into<Length>, Y: Into<Length>>(mut self, x: X, y: Y) -> Self {
        self.data.push(Command::MoveTo(x.into(), y.into()));
        self
    }
    pub fn move_rel<X: Into<Length>, Y: Into<Length>>(mut self, dx: X, dy: Y) -> Self {
        self.data.push(Command::MoveBy(dx.into(), dy.into()));
        self
    }
    pub fn line_abs<X: Into<Length>, Y: Into<Length>>(mut self, x: X, y: Y) -> Self {
        self.data.push(Command::LineTo(x.into(), y.into()));
        self
    }
    pub fn line_rel<X: Into<Length>, Y: Into<Length>>(mut self, dx: X, dy: Y) -> Self {
        self.data.push(Command::LineBy(dx.into(), dy.into()));
        self
    }
    pub fn quad_curve_abs<X1: Into<Length>, Y1: Into<Length>, X: Into<Length>, Y: Into<Length>>(mut self, x1: X1, y1: Y1, x: X, y: Y) -> Self {
        self.data.push(Command::QCurTo(x1.into(), y1.into(), x.into(), y.into()));
        self
    }
    pub fn quad_curve_rel<X1: Into<Length>, Y1: Into<Length>, X: Into<Length>, Y: Into<Length>>(mut self, dx1: X1, dy1: Y1, dx: X, dy: Y) -> Self {
        self.data.push(Command::QCurBy(dx1.into(), dy1.into(), dx.into(), dy.into()));
        self
    }
    pub fn cubic_curve_abs<X1: Into<Length>, Y1: Into<Length>, X2: Into<Length>, Y2: Into<Length>, X: Into<Length>, Y: Into<Length>>(mut self, x1: X1, y1: Y1, x2: X2, y2: Y2, x: X, y: Y) -> Self {
        self.data.push(Command::CCurTo(x1.into(), y1.into(), x2.into(), y2.into(), x.into(), y.into()));
        self
    }
    pub fn cubic_curve_rel<X1: Into<Length>, Y1: Into<Length>, X2: Into<Length>, Y2: Into<Length>, X: Into<Length>, Y: Into<Length>>(mut self, dx1: X1, dy1: Y1, dx2: X2, dy2: Y2, dx: X, dy: Y) -> Self {
        self.data.push(Command::CCurBy(dx1.into(), dy1.into(), dx2.into(), dy2.into(), dx.into(), dy.into()));
        self
    }
    pub fn close(mut self) -> Self {
        self.data.push(Command::Close);
        self
    }
}

impl AbsPos for Path {
    type Output = SvgPath;
    fn set_abs_pos<X: Into<Length>, Y: Into<Length>>(self, x: X, y: Y, scale: &Scale) -> Self::Output {
        let data = self.data.into_iter().fold(
            SvgData::new().move_to((x.into(), y.into())),
            |data, com| {
                match com {
                    Command::MoveTo(x, y) => data.move_to((x * scale.x, y * scale.y)),
                    Command::MoveBy(x, y) => data.move_by((x * scale.x, y * scale.y)),
                    Command::LineTo(x, y) => data.line_to((x * scale.x, y * scale.y)),
                    Command::LineBy(x, y) => data.line_by((x * scale.x, y * scale.y)),
                    Command::QCurTo(x1, y1, x, y) => data.quadratic_curve_to((x1 * scale.x, y1 * scale.y, x * scale.x, y * scale.y)),
                    Command::QCurBy(x1, y1, x, y) => data.quadratic_curve_by((x1 * scale.x, y1 * scale.y, x * scale.x, y * scale.y)),
                    Command::CCurTo(x1, y1, x2, y2, x, y) => data.cubic_curve_to((x1 * scale.x, y1 * scale.y, x2 * scale.x, y2 * scale.y, x * scale.x, y * scale.y)),
                    Command::CCurBy(x1, y1, x2, y2, x, y) => data.cubic_curve_by((x1 * scale.x, y1 * scale.y, x2 * scale.x, y2 * scale.y, x * scale.x, y * scale.y)),
                    Command::Close => data.close(),
                }
            });
        self.path.set("d", data)
    }
}

crate::attribute::fill::implement_fill! { Path, path }
crate::attribute::stroke::implement_stroke! { Path, path }
crate::attribute::stroke_width::implement_stroke_width! { Path, path }

#[cfg(test)]
mod path_tests {
    use crate::chart::Chart;
    use super::Path;
    use crate::attribute::{ Fill, Stroke, StrokeWidth };
    #[test]
    fn path_test1() {
        let ch = Chart::new(0, 0, 100, 100);
        let p1 = Path::new()
            .fill("none")
            .stroke("red")
            .stroke_width(3)
            .move_rel(10, 70)
            .line_rel(10, -10)
            .line_rel(10, -20)
            .line_rel(10, 15)
            .line_rel(20, 10);

        let ch = ch
            .draw(p1.clone(), 0, 0)
            .draw(p1, 10, 10);
        println!("{}", ch);
    }
}
