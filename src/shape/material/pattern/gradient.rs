use crate::{color::Color, transformation::Transformation, tuple::Tuple};

use super::Pattern;

#[derive(Debug, Clone, Default)]
pub struct GradientPattern {
    color_a: Color,
    color_b: Color,
    transformation: Transformation,
}

impl GradientPattern {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Self {
            color_a,
            color_b,
            transformation: Transformation::identity(),
        }
    }
}

impl Pattern for GradientPattern {
    fn color_at(&self, point: Tuple) -> Color {
        let distance = self.color_b - self.color_a;
        let fraction = point.x() - point.x().floor();

        self.color_a + distance * fraction
    }

    fn set_transformation(&mut self, transformation: Transformation) {
        self.transformation = transformation;
    }

    fn transformation(&self) -> Transformation {
        self.transformation.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Colors;

    use super::*;

    #[test]
    fn a_gradient_lindearly_interpolates_between_colors() {
        let pattern = GradientPattern::new(Colors::White.into(), Colors::Black.into());

        assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 0.0)), Color::new(1.0, 1.0, 1.0));
        assert_eq!(pattern.color_at(Tuple::point(0.25, 0.0, 0.0)), Color::new(0.75, 0.75, 0.75));
        assert_eq!(pattern.color_at(Tuple::point(0.5, 0.0, 0.0)), Color::new(0.5, 0.5, 0.5));
        assert_eq!(pattern.color_at(Tuple::point(0.75, 0.0, 0.0)), Color::new(0.25, 0.25, 0.25));
    }
}