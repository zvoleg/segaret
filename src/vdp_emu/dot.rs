use spriter::Color;

#[derive(PartialEq, Clone, Copy)]
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

impl Default for Dot {
    fn default() -> Self {
        Self { color: Default::default(), priority: Priority::Low }
    }
}