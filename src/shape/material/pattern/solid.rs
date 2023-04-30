use crate::color::Color;

use super::Pattern;

#[derive(Debug, Clone, Copy)]
pub struct Solid {
    color: Color,
}

impl Solid {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Pattern for Solid {
    fn color_at(&self, _point: crate::tuple::Tuple) -> Color {
        self.color()
    }
}
