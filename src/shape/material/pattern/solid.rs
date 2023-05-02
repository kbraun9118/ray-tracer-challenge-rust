use crate::{color::Color, transformation::Transformation, tuple::Tuple};

use super::Pattern;

#[derive(Debug, Clone)]
pub struct SolidPattern {
    color: Color,
}

impl SolidPattern {
    pub fn new(color: Color) -> Self {
        Self {
            color
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Pattern for SolidPattern {
    fn color_at(&self, _point: Tuple) -> Color {
        self.color()
    }

    fn set_transformation(&mut self, _transformation: Transformation) {
    }

    fn transformation(&self) -> Transformation {
        Transformation::identity()
    }
}
