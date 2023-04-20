use std::fmt::Debug;

use super::ray::Ray;
use crate::{transformation::Transformation, tuple::Tuple};
use uuid::Uuid;

pub trait Shape: Debug {
    fn id(&self) -> Uuid;
    fn intersects(&self, ray: Ray) -> Vec<f64>;
    fn transformation(&self) -> &Option<Transformation>;
    fn with_transformation(&mut self, transformation: Transformation);
}

#[derive(Debug)]
pub struct Sphere {
    id: Uuid,
    center: Tuple,
    radius: f64,
    transformation: Option<Transformation>,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            center: Tuple::origin(),
            radius: 1.0,
            transformation: None,
        }
    }
}

impl Shape for Sphere {
    fn intersects(&self, ray: Ray) -> Vec<f64> {
        let transformation = self.transformation().as_ref().map_or_else(Transformation::default, |t| t.clone());

        let ray = transformation.inverse().unwrap() * ray;

        let sphere_to_ray = ray.origin() - self.center;

        let a = ray.direciton() * ray.direciton();
        let b = (ray.direciton() * sphere_to_ray) * 2.0;
        let c = sphere_to_ray * sphere_to_ray - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            vec![
                (-b - discriminant.sqrt()) / (2.0 * a),
                (-b + discriminant.sqrt()) / (2.0 * a),
            ]
        }
    }

    fn id(&self) -> Uuid {
        self.id
    }

    fn transformation(&self) -> &Option<Transformation> {
        &self.transformation
    }

    fn with_transformation(&mut self, transformation: Transformation) {
        self.transformation = Some(transformation);
    }
}

impl PartialEq for &dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::try_new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = Sphere::new();

        let xs = s.intersects(r);

        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0]);
        assert_eq!(6.0, xs[1]);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::try_new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = Sphere::new();

        let xs = s.intersects(r);

        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0]);
        assert_eq!(5.0, xs[1]);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::try_new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = Sphere::new();

        let xs = s.intersects(r);

        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::try_new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = Sphere::new();

        let xs = s.intersects(r);

        assert_eq!(2, xs.len());
        assert_eq!(-1.0, xs[0]);
        assert_eq!(1.0, xs[1]);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::try_new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = Sphere::new();

        let xs = s.intersects(r);

        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0]);
        assert_eq!(-4.0, xs[1]);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::try_new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let mut s = Sphere::new();

        s.with_transformation(Transformation::identity().scale(2.0, 2.0, 2.0));

        let xs = s.intersects(r);

        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0]);
        assert_eq!(7.0, xs[1]);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::try_new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let mut s = Sphere::new();

        s.with_transformation(Transformation::identity().translation(5.0, 0.0, 0.0));

        let xs = s.intersects(r);

        assert_eq!(0, xs.len());
    }
}
