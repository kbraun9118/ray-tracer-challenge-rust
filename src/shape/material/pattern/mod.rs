use crate::{color::Color, shape::ShapeContainer, transformation::Transformation, tuple::Tuple};
use std::fmt::Debug;

pub mod checker;
pub mod gradient;
pub mod ring;
pub mod solid;
pub mod stripes;

pub trait Pattern: Debug {
    fn color_at(&self, point: Tuple) -> Color;
    fn set_transformation(&mut self, transformation: Transformation);
    fn transformation(&self) -> Transformation;

    fn color_at_object(&self, shape: ShapeContainer, point: Tuple) -> Color {
        let object_point = shape.read().unwrap().transformation().inverse().unwrap() * point;
        let pattern_point = self.transformation().inverse().unwrap() * object_point;
        self.color_at(pattern_point)
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub(crate) struct TestPattern {
    transformation: Transformation,
}

impl Pattern for TestPattern {
    fn color_at(&self, point: Tuple) -> Color {
        Color::new(point.x(), point.y(), point.z())
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
    use crate::shape::{sphere::Sphere, Shape};

    use super::*;

    #[test]
    fn the_default_pattern_tranformation() {
        assert_eq!(
            TestPattern::default().transformation(),
            Transformation::identity()
        )
    }

    #[test]
    fn assigning_a_tranformation() {
        let mut pattern = TestPattern::default();
        pattern.set_transformation(Transformation::identity().translation(1.0, 2.0, 3.0));

        assert_eq!(
            pattern.transformation(),
            Transformation::identity().translation(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn a_pattern_with_an_object_tranformation() {
        let mut object = Sphere::new();
        object.set_transformation(Transformation::identity().scale(2.0, 2.0, 2.0));
        let pattern = TestPattern::default();
        let c = pattern.color_at_object(object.into(), Tuple::point(2.0, 3.0, 4.0));

        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn a_pattern_with_a_pattern_tranformation() {
        let object = Sphere::new();
        let mut pattern = TestPattern::default();
        pattern.set_transformation(Transformation::identity().scale(2.0, 2.0, 2.0));
        let c = pattern.color_at_object(object.into(), Tuple::point(2.0, 3.0, 4.0));

        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let mut object = Sphere::new();
        object.set_transformation(Transformation::identity().scale(2.0, 2.0, 2.0));
        let mut pattern = TestPattern::default();
        pattern.set_transformation(Transformation::identity().translation(0.5, 1.0, 1.5));
        let c = pattern.color_at_object(object.into(), Tuple::point(2.5, 3.0, 3.5));

        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
