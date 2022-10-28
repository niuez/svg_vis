use super::super::Color;
use super::{ ColorMap, lin_interpolate };

pub struct Jet;

impl Jet {
    pub fn new() -> Self {
        Jet
    }
}

const JET_DATA: [(f32, f32, f32); 9] =
    [
            ( 0.0, 0.0, 0.5 ),
            ( 0.0, 0.0, 1.0 ),
            ( 0.0, 0.5, 1.0 ),
            ( 0.0, 1.0, 1.0 ),
            ( 0.5, 1.0, 0.5 ),
            ( 1.0, 1.0, 0.0 ),
            ( 1.0, 0.5, 0.0 ),
            ( 1.0, 0.0, 0.0 ),
            ( 0.5, 0.0, 0.0 )
    ];

impl ColorMap for Jet {
    fn cmap(&self, v: f32) -> Color {
        let (r, g, b) = lin_interpolate(v, &JET_DATA);
        Color::from_rgb(r, g, b)
    }
}

#[cfg(test)]
mod jet_tests {
    use super::Jet;
    use crate::element::Circle;
    use crate::attribute::Fill;
    use crate::literal::color::colormap::ColorMap;
    use crate::chart::Chart;
    #[test]
    fn jet_test() {
        let ch = Chart::new(0, 0, 100, 100);
        let jet = Jet::new();
        let ch = (0..10)
            .map(|i| (i, Circle::new().radius(5).fill(jet.cmap(i as f32 / 10.0))))
            .fold(ch, |ch, (i, c)| ch.draw(c, 10 * i + 5, 50));
        println!("{}", ch);

        /*
        for i in 0..10 {
            let c = Circle::new()
                .radius(5)
                .fill(jet.cmap(i as f32 / 10.0));
            ch = ch.draw(c, 10 * i + 5, 50);
        }
        */
    }
}
