use svg::node::Value;

pub enum TextAnchorValue {
    Start,
    Middle,
    End,
}

impl From<TextAnchorValue> for Value {
    fn from(value: TextAnchorValue) -> Self {
        match value {
            TextAnchorValue::Start => "start".into(),
            TextAnchorValue::Middle => "middle".into(),
            TextAnchorValue::End => "end".into(),
        }
    }
}

pub trait TextAnchor {
    fn text_anchor(self, value: TextAnchorValue) -> Self;
}

macro_rules! implement_text_anchor {
    ($T:ty, $svg:ident) => (
        impl TextAnchor for $T {
            fn text_anchor(mut self, value: TextAnchorValue) -> Self {
                self.$svg = self.$svg.set("text-anchor", value);
                self
            }
        }
    );
}

pub(crate) use implement_text_anchor;
