
pub trait Title {
    fn title<S: Into<String>>(self, s: S) -> Self;
}

macro_rules! implement_title {
    ($T:ty, $svg:ident) => (
        impl Title for $T {
            fn title<S: Into<String>>(mut self, s: S) -> Self {
                let text = svg::node::Text::new(s);
                let title = svg::node::element::Title::new()
                    .add(text);
                self.$svg = self.$svg.add(title);
                self
            }
        }
    );
}

pub(crate) use implement_title;
