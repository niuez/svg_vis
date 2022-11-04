pub trait Title {
    fn title<S: ToString>(self, s: S) -> Self;
}

macro_rules! implement_title {
    ($T:ty, $svg:ident) => (
        impl Title for $T {
            fn title<S: ToString>(mut self, s: S) -> Self {
                self.$svg = self.$svg.set("title", s.to_string());
                self
            }
        }
    );
}

pub(crate) use implement_title;
