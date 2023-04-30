use crate::{color::Color, tuple::Tuple};

use super::Pattern;

#[derive(Debug, Clone, Copy)]
pub struct Stripes {
    color_a: Color,
    color_b: Color,
}

impl Stripes {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }

    pub fn color_a(&self) -> Color {
        self.color_a
    }

    pub fn color_b(&self) -> Color {
        self.color_b
    }
}

impl Pattern for Stripes {
    fn color_at(&self, point: Tuple) -> Color {
        if point.x().floor() % 2.0 == 0.0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Colors;

    use super::*;

    #[test]
    fn creating_a_stripe_pattern() {
        let stripes = Stripes::new(Colors::White.into(), Colors::Black.into());

        assert_eq!(stripes.color_a(), Colors::White.into());
        assert_eq!(stripes.color_b(), Colors::Black.into());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let stripes = Stripes::new(Colors::White.into(), Colors::Black.into());

        assert_eq!(
            stripes.color_at(Tuple::point(0.0, 0.0, 0.0)),
            Colors::White.into()
        );
        assert_eq!(
            stripes.color_at(Tuple::point(0.0, 1.0, 0.0)),
            Colors::White.into()
        );
        assert_eq!(
            stripes.color_at(Tuple::point(0.0, 2.0, 0.0)),
            Colors::White.into()
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let stripes = Stripes::new(Colors::White.into(), Colors::Black.into());

        assert_eq!(
            stripes.color_at(Tuple::point(0.0, 0.0, 0.0)),
            Colors::White.into()
        );
        assert_eq!(
            stripes.color_at(Tuple::point(0.0, 0.0, 1.0)),
            Colors::White.into()
        );
        assert_eq!(
            stripes.color_at(Tuple::point(0.0, 0.0, 2.0)),
            Colors::White.into()
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let stripes = Stripes::new(Colors::White.into(), Colors::Black.into());

        assert_eq!(
            stripes.color_at(Tuple::point(0.0, 0.0, 0.0)),
            Colors::White.into()
        );
        assert_eq!(
            stripes.color_at(Tuple::point(0.9, 0.0, 0.0)),
            Colors::White.into()
        );
        assert_eq!(
            stripes.color_at(Tuple::point(1.0, 0.0, 0.0)),
            Colors::Black.into()
        );
        assert_eq!(
            stripes.color_at(Tuple::point(-0.1, 0.0, 0.0)),
            Colors::Black.into()
        );
        assert_eq!(
            stripes.color_at(Tuple::point(-1.0, 0.0, 0.0)),
            Colors::Black.into()
        );
        assert_eq!(
            stripes.color_at(Tuple::point(-1.1, 0.0, 0.0)),
            Colors::White.into()
        );
    }
}
