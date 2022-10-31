pub trait ID {
    fn id<S: Into<String>>(self, id: S) -> Self;
}

macro_rules! implement_id {
    ($T:ty, $svg:ident) => (
        impl ID for $T {
            fn id<S: Into<String>>(mut self, id: S) -> Self {
                self.$svg = self.$svg.set("id", id.into());
                self
            }
        }
    );
}

pub(crate) use implement_id;
