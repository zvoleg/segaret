use spriter::Color;

#[derive(PartialEq)]
pub(crate) enum Priority {
    High,
    Low,
}

pub(crate) struct Dot {
    pub(crate) color: Option<Color>,
    pub(crate) priority: Priority,
}

impl Dot {
    pub(crate) fn new(color: Option<Color>, priority: Priority) -> Self {
        Self { color, priority }
    }
}
