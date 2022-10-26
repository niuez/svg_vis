use svg::node::Value;

pub enum DominantBaselineValue {
    Auto,
    TextBottom,
    Alphabetic,
    Ideographic,
    Middle,
    Central,
    Mathematical,
    Hanging,
    TextTop,
}

impl From<DominantBaselineValue> for Value {
    fn from(value: DominantBaselineValue) -> Self {
        match value {
            DominantBaselineValue::Auto => "auto".into(),
            DominantBaselineValue::TextBottom => "text-bottom".into(),
            DominantBaselineValue::Alphabetic => "alphabetic".into(),
            DominantBaselineValue::Ideographic => "ideographic".into(),
            DominantBaselineValue::Middle => "middle".into(),
            DominantBaselineValue::Central => "central".into(),
            DominantBaselineValue::Mathematical => "mathematical".into(),
            DominantBaselineValue::Hanging => "hanging".into(),
            DominantBaselineValue::TextTop => "text-top".into(),
        }
    }
}

pub trait DominantBaseline {
    fn dominant_baseline(self, value: DominantBaselineValue) -> Self;
}

macro_rules! implement_dominant_baseline {
    ($T:ty, $svg:ident) => (
        impl DominantBaseline for $T {
            fn dominant_baseline(mut self, value: DominantBaselineValue) -> Self {
                self.$svg = self.$svg.set("dominant-baseline", value);
                self
            }
        }
    );
}

pub(crate) use implement_dominant_baseline;
