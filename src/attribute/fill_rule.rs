pub enum FillRuleValue {
    NonZero,
    EvenOdd,
}

impl FillRuleValue {
    pub fn to_str(&self) -> &str {
        match *self {
            Self::NonZero => "nonzero",
            Self::EvenOdd => "evenodd",
        }
    }
}

pub trait FillRule {
    fn fill_rule(self, value: FillRuleValue) -> Self;
}

macro_rules! implement_fillrule {
    ($T:ty, $svg:ident) => (
        impl FillRule for $T {
            fn fill_rule(mut self, value: FillRuleValue) -> Self {
                self.$svg = self.$svg.set("fill-rule", value.to_str());
                self
            }
        }
    );
}

pub(crate) use implement_fillrule;
