use std::{
    f64::{INFINITY, NEG_INFINITY},
    mem::swap,
};

use crate::{intersection::ray::Ray, transformation::Transformation, tuple::Tuple, util};

#[derive(Debug)]
pub struct BoundedBox {
    min: Tuple,
    max: Tuple,
}

fn check_axis(origin: f64, direction: f64, min: f64, max: f64) -> (f64, f64) {
    let tmin_numerator = min - origin;
    let tmax_numerator = max - origin;

    let (mut tmin, mut tmax) = if direction.abs() >= util::EPSILON {
        (tmin_numerator / direction, tmax_numerator / direction)
    } else {
        (tmin_numerator * INFINITY, tmax_numerator * INFINITY)
    };

    if tmin > tmax {
        swap(&mut tmin, &mut tmax);
    }

    (tmin, tmax)
}

impl BoundedBox {
    pub(crate) fn new(min: Tuple, max: Tuple) -> Self {
        Self { min, max }
    }

    pub(crate) fn empty() -> Self {
        Self {
            min: Tuple::point(INFINITY, INFINITY, INFINITY),
            max: Tuple::point(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY),
        }
    }

    pub(crate) fn intersects(&self, ray: Ray) -> bool {
        let (xtmin, xtmax) = check_axis(
            ray.origin().x(),
            ray.direction().x(),
            self.min.x(),
            self.max.x(),
        );
        let (ytmin, ytmax) = check_axis(
            ray.origin().y(),
            ray.direction().y(),
            self.min.y(),
            self.max.y(),
        );
        let (ztmin, ztmax) = check_axis(
            ray.origin().z(),
            ray.direction().z(),
            self.min.z(),
            self.max.z(),
        );

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin > tmax {
            false
        } else {
            true
        }
    }

    // pub(crate) fn min(&self) -> Tuple {
    //     self.min
    // }
    //
    // pub(crate) fn max(&self) -> Tuple {
    //     self.max
    // }

    pub(crate) fn add_point(&mut self, point: Tuple) {
        self.min = Tuple::point(
            self.min.x().min(point.x()),
            self.min.y().min(point.y()),
            self.min.z().min(point.z()),
        );
        self.max = Tuple::point(
            self.max.x().max(point.x()),
            self.max.y().max(point.y()),
            self.max.z().max(point.z()),
        );
    }

    pub(crate) fn add_box(&mut self, other: Self) {
        self.add_point(other.min);
        self.add_point(other.max);
    }

    fn contains_point(&self, point: Tuple) -> bool {
        self.min.x() <= point.x()
            && point.x() <= self.max.x()
            && self.min.y() <= point.y()
            && point.y() <= self.max.y()
            && self.min.z() <= point.z()
            && point.z() <= self.max.z()
    }

    fn contains_box(&self, other: Self) -> bool {
        self.contains_point(other.min) && self.contains_point(other.max)
    }

    pub(crate) fn transform(&self, transformation: Transformation) -> Self {
        let p0 = self.min;
        let p1 = Tuple::point(self.min.x(), self.min.y(), self.max.z());
        let p2 = Tuple::point(self.min.x(), self.max.y(), self.max.z());
        let p3 = Tuple::point(self.max.x(), self.min.y(), self.max.z());
        let p4 = Tuple::point(self.max.x(), self.max.y(), self.min.z());
        let p5 = Tuple::point(self.max.x(), self.min.y(), self.min.z());
        let p6 = Tuple::point(self.min.x(), self.max.y(), self.min.z());
        let p7 = self.max;

        let mut new = BoundedBox::empty();

        for p in [p0, p1, p2, p3, p4, p5, p6, p7] {
            new.add_point(transformation.clone() * p);
        }

        new
    }
}

#[cfg(test)]
mod test {

    use core::f64;

    use crate::shape::{
        cone::Cone,
        cylinder::Cylinder,
        group::{Group, GroupContainer},
        sphere::Sphere,
        Shape,
    };

    use super::*;

    #[test]
    fn creating_an_empty_bounding_box() {
        let bbox = BoundedBox::empty();

        assert_eq!(bbox.min, Tuple::point(INFINITY, INFINITY, INFINITY));
        assert_eq!(
            bbox.max,
            Tuple::point(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY)
        );
    }

    #[test]
    fn adding_points_to_an_empty_bounding_box() {
        let mut bbox = BoundedBox::empty();
        bbox.add_point(Tuple::point(-5.0, 2.0, 0.0));
        bbox.add_point(Tuple::point(7.0, 0.0, -3.0));

        assert_eq!(bbox.min, Tuple::point(-5.0, 0.0, -3.0));
        assert_eq!(bbox.max, Tuple::point(7.0, 2.0, 0.0));
    }

    #[test]
    fn a_bounded_cone_has_a_bounding_box() {
        let mut shape = Cone::new();
        shape.set_minimum(-5.0);
        shape.set_maximum(3.0);
        let bbox = shape.bounds();

        assert_eq!(bbox.min, Tuple::point(-5.0, -5.0, -5.0));
        assert_eq!(bbox.max, Tuple::point(5.0, 3.0, 5.0));
    }

    #[test]
    fn adding_one_bounding_box_to_another() {
        let mut box1 = BoundedBox::new(Tuple::point(-5.0, -2.0, 0.0), Tuple::point(7.0, 4.0, 4.0));
        let box2 = BoundedBox::new(Tuple::point(8.0, -7.0, -2.0), Tuple::point(14.0, 2.0, 8.0));

        box1.add_box(box2);

        assert_eq!(box1.min, Tuple::point(-5.0, -7.0, -2.0));
        assert_eq!(box1.max, Tuple::point(14.0, 4.0, 8.0));
    }

    #[test]
    fn checking_to_see_if_a_box_contains_a_given_point() {
        let exs = vec![
            (Tuple::point(5.0, -2.0, 0.0), true),
            (Tuple::point(11.0, 4.0, 7.0), true),
            (Tuple::point(8.0, 1.0, 3.0), true),
            (Tuple::point(3.0, 0.0, 3.0), false),
            (Tuple::point(8.0, -4.0, 3.0), false),
            (Tuple::point(8.0, 1.0, -1.0), false),
            (Tuple::point(13.0, 1.0, 3.0), false),
            (Tuple::point(8.0, 5.0, 3.0), false),
            (Tuple::point(8.0, 1.0, 8.0), false),
        ];

        let bbox = BoundedBox::new(Tuple::point(5.0, -2.0, 0.0), Tuple::point(11.0, 4.0, 7.0));

        for (point, result) in exs {
            assert_eq!(bbox.contains_point(point), result);
        }
    }

    #[test]
    fn checking_if_a_box_contains_a_given_box() {
        let exs = vec![
            (
                Tuple::point(5.0, -2.0, 0.0),
                Tuple::point(11.0, 4.0, 7.0),
                true,
            ),
            (
                Tuple::point(6.0, -1.0, 1.0),
                Tuple::point(10.0, 3.0, 6.0),
                true,
            ),
            (
                Tuple::point(4.0, -3.0, -1.0),
                Tuple::point(10.0, 3.0, 6.0),
                false,
            ),
            (
                Tuple::point(6.0, -1.0, 1.0),
                Tuple::point(12.0, 5.0, 8.0),
                false,
            ),
        ];

        let box0 = BoundedBox::new(Tuple::point(5.0, -2.0, 0.0), Tuple::point(11.0, 4.0, 7.0));

        for (min, max, result) in exs {
            let box1 = BoundedBox::new(min, max);

            assert_eq!(box0.contains_box(box1), result);
        }
    }

    #[test]
    fn transforming_a_bounded_box() {
        let bbox = BoundedBox::new(Tuple::point(-1.0, -1.0, -1.0), Tuple::point(1.0, 1.0, 1.0));
        let transformation = Transformation::identity()
            .rotate_y(f64::consts::PI / 4.0)
            .rotate_x(f64::consts::PI / 4.0);
        let box2 = bbox.transform(transformation);

        assert_eq!(box2.min, Tuple::point(-1.41421, -1.7071, -1.7071));
        assert_eq!(box2.max, Tuple::point(1.41421, 1.7071, 1.7071));
    }

    #[test]
    fn querying_a_shapes_bounded_box_in_its_parent_space() {
        let mut s = Sphere::new();
        s.set_transformation(
            Transformation::identity()
                .scale(0.5, 2.0, 4.0)
                .translation(1.0, -3.0, 5.0),
        );

        let bbox = s.parent_space_bounds();

        assert_eq!(bbox.min, Tuple::point(0.5, -5.0, 1.0));
        assert_eq!(bbox.max, Tuple::point(1.5, -1.0, 9.0));
    }

    #[test]
    fn a_group_has_a_bounded_box_that_contains_its_children() {
        let mut s = Sphere::new();
        s.set_transformation(
            Transformation::identity()
                .scale(2.0, 2.0, 2.0)
                .translation(2.0, 5.0, -3.0),
        );
        let mut c = Cylinder::new();
        c.set_minimum(-2.0);
        c.set_maximum(2.0);
        c.set_transformation(
            Transformation::identity()
                .scale(0.5, 1.0, 0.5)
                .translation(-4.0, -1.0, 4.0),
        );
        let shape = GroupContainer::from(Group::new());
        shape.add_child(s.into());
        shape.add_child(c.into());

        let bounds = shape.read().unwrap().bounds();

        assert_eq!(bounds.min, Tuple::point(-4.5, -3.0, -5.0));
        assert_eq!(bounds.max, Tuple::point(4.0, 7.0, 4.5));
    }

    #[test]
    fn intersecting_a_ray_with_a_bounding_box_at_the_origin() {
        let exs = vec![
            (
                Tuple::point(5.0, 0.5, 0.0),
                Tuple::vector(-1.0, 0.0, 0.0),
                true,
            ),
            (
                Tuple::point(-5.0, 0.5, 0.0),
                Tuple::vector(1.0, 0.0, 0.0),
                true,
            ),
            (
                Tuple::point(0.5, 5.0, 0.0),
                Tuple::vector(0.0, -1.0, 0.0),
                true,
            ),
            (
                Tuple::point(0.5, -5.0, 0.0),
                Tuple::vector(0.0, 1.0, 0.0),
                true,
            ),
            (
                Tuple::point(0.5, 0.0, 5.0),
                Tuple::vector(0.0, 0.0, -1.0),
                true,
            ),
            (
                Tuple::point(0.5, 0.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0),
                true,
            ),
            (
                Tuple::point(0.0, 0.5, 0.0),
                Tuple::vector(0.0, 0.0, 1.0),
                true,
            ),
            (
                Tuple::point(-2.0, 0.0, 0.0),
                Tuple::vector(2.0, 4.0, 6.0),
                false,
            ),
            (
                Tuple::point(0.0, -2.0, 0.0),
                Tuple::vector(6.0, 2.0, 4.0),
                false,
            ),
            (
                Tuple::point(0.0, 0.0, -2.0),
                Tuple::vector(4.0, 6.0, 2.0),
                false,
            ),
            (
                Tuple::point(2.0, 0.0, 2.0),
                Tuple::vector(0.0, 0.0, -1.0),
                false,
            ),
            (
                Tuple::point(0.0, 2.0, 2.0),
                Tuple::vector(0.0, -1.0, 0.0),
                false,
            ),
            (
                Tuple::point(2.0, 2.0, 0.0),
                Tuple::vector(-1.0, 0.0, 0.0),
                false,
            ),
        ];
        let bbox = BoundedBox::new(Tuple::point(-1.0, -1.0, -1.0), Tuple::point(1.0, 1.0, 1.0));

        for (origin, direction, result) in exs {
            let direction = direction.normalize();
            let r = Ray::new(origin, direction);

            assert_eq!(bbox.intersects(r), result);
        }
    }
}
