use crate::{color::Color, transformation::Transformation, tuple::Tuple, util::eq_f64};

use super::Pattern;

#[derive(Debug, Clone)]
pub struct CheckerPattern {
    color_a: Color,
    color_b: Color,
    transformation: Transformation,
}

impl CheckerPattern {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Self {
            color_a,
            color_b,
            transformation: Transformation::identity(),
        }
    }
}

impl Pattern for CheckerPattern {
    fn color_at(&self, point: Tuple) -> Color {
        if eq_f64((point.x().floor() + point.y().floor() + point.z().floor()) % 2.0, 0.0) {
            self.color_a
        } else {
            self.color_b
        }
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
    fn checkers_should_repeat_in_x() {
        let pattern = CheckerPattern::new(Colors::White.into(), Colors::Black.into());

        assert_eq!(
            pattern.color_at(Tuple::point(0.0, 0.0, 0.0)),
            Colors::White.into()
        );
        assert_eq!(
            pattern.color_at(Tuple::point(0.99, 0.0, 0.0)),
            Colors::White.into()
        );
        assert_eq!(
            pattern.color_at(Tuple::point(1.01, 0.0, 0.0)),
            Colors::Black.into()
        );
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = CheckerPattern::new(Colors::White.into(), Colors::Black.into());

        assert_eq!(
            pattern.color_at(Tuple::point(0.0, 0.0, 0.0)),
            Colors::White.into()
        );
        assert_eq!(
            pattern.color_at(Tuple::point(0.0, 0.99, 0.0)),
            Colors::White.into()
        );
        assert_eq!(
            pattern.color_at(Tuple::point(0.0, 1.01, 0.0)),
            Colors::Black.into()
        );
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = CheckerPattern::new(Colors::White.into(), Colors::Black.into());

        assert_eq!(
            pattern.color_at(Tuple::point(0.0, 0.0, 0.0)),
            Colors::White.into()
        );
        assert_eq!(
            pattern.color_at(Tuple::point(0.0, 0.0, 0.99)),
            Colors::White.into()
        );
        assert_eq!(
            pattern.color_at(Tuple::point(0.0, 0.0, 1.01)),
            Colors::Black.into()
        );
    }
}
