use crate::{color::Color, transformation::Transformation, tuple::Tuple, util::eq_f64};

use super::Pattern;

#[derive(Debug, Clone)]
pub struct RingPattern {
    color_a: Color,
    color_b: Color,
    transformation: Transformation,
}

impl RingPattern {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Self {
            color_a,
            color_b,
            transformation: Transformation::identity(),
        }
    }
}

impl Pattern for RingPattern {
    fn color_at(&self, point: Tuple) -> Color {
        if eq_f64((point.x().powi(2) + point.z().powi(2)).sqrt().floor() % 2.0, 0.0) {
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
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = RingPattern::new(Colors::White.into(), Colors::Black.into());

        assert_eq!(
            pattern.color_at(Tuple::point(0.0, 0.0, 0.0)),
            Colors::White.into()
        );
        assert_eq!(
            pattern.color_at(Tuple::point(1.0, 0.0, 0.0)),
            Colors::Black.into()
        );
        assert_eq!(
            pattern.color_at(Tuple::point(0.0, 0.0, 1.0)),
            Colors::Black.into()
        );
        assert_eq!(
            pattern.color_at(Tuple::point(0.708, 0.0, 0.708)),
            Colors::Black.into()
        );
    }
}
