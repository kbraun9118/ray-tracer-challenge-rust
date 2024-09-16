use crate::{color::Color, transformation::Transformation, tuple::Tuple, util::eq_f64};

use super::Pattern;

#[derive(Debug, Clone)]
pub struct StripePattern {
    color_a: Color,
    color_b: Color,
    transformation: Transformation,
}

impl StripePattern {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Self {
            color_a,
            color_b,
            transformation: Transformation::identity(),
        }
    }

    pub fn color_a(&self) -> Color {
        self.color_a
    }

    pub fn color_b(&self) -> Color {
        self.color_b
    }
}

impl Pattern for StripePattern {
    fn color_at(&self, point: Tuple) -> Color {
        if eq_f64(point.x().floor() % 2.0, 0.0) {
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
    use crate::{
        color::Colors,
        shape::{sphere::Sphere, Shape},
    };

    use super::*;

    #[test]
    fn creating_a_stripe_pattern() {
        let stripes = StripePattern::new(Colors::White.into(), Colors::Black.into());

        assert_eq!(stripes.color_a(), Colors::White.into());
        assert_eq!(stripes.color_b(), Colors::Black.into());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let stripes = StripePattern::new(Colors::White.into(), Colors::Black.into());

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
        let stripes = StripePattern::new(Colors::White.into(), Colors::Black.into());

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
        let stripes = StripePattern::new(Colors::White.into(), Colors::Black.into());

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

    #[test]
    fn stripes_with_an_object_tranformation() {
        let mut object = Sphere::new();
        object.set_transformation(Transformation::identity().scale(2.0, 2.0, 2.0));
        let pattern = StripePattern::new(Colors::White.into(), Colors::Black.into());
        let c = pattern.color_at_object(object.into(), Tuple::point(1.5, 0.0, 0.0));

        assert_eq!(c, Colors::White.into());
    }

    #[test]
    fn stripes_with_a_pattern_tranformation() {
        let object = Sphere::new();
        let mut pattern = StripePattern::new(Colors::White.into(), Colors::Black.into());
        pattern.set_transformation(Transformation::identity().scale(2.0, 2.0, 2.0));
        let c = pattern.color_at_object(object.into(), Tuple::point(1.5, 0.0, 0.0));

        assert_eq!(c, Colors::White.into());
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object = Sphere::new();
        object.set_transformation(Transformation::identity().scale(2.0, 2.0, 2.0));
        let mut pattern = StripePattern::new(Colors::White.into(), Colors::Black.into());
        pattern.set_transformation(Transformation::identity().translation(0.5, 0.0, 0.0));
        let c = pattern.color_at_object(object.into(), Tuple::point(2.5, 0.0, 0.0));

        assert_eq!(c, Colors::White.into());
    }
}
