use std::fmt::Debug;

use crate::{intersection::ray::Ray, transformation::Transformation, tuple::Tuple};
use uuid::Uuid;

use super::{Shape, material::Material};

#[derive(Debug)]
pub struct Sphere {
    id: Uuid,
    center: Tuple,
    transformation: Option<Transformation>,
    material: Material
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            center: Tuple::origin(),
            transformation: None,
            material: Material::new(),
        }
    }
}

impl Shape for Sphere {
    fn intersects(&self, ray: Ray) -> Vec<f64> {
        let ray = self.transformation().inverse().unwrap() * ray;

        let sphere_to_ray = ray.origin() - self.center;

        let a = ray.direction() * ray.direction();
        let b = (ray.direction() * sphere_to_ray) * 2.0;
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

    fn transformation(&self) -> Transformation {
        self.transformation
            .as_ref()
            .map_or_else(Transformation::default, |t| t.clone())
    }

    fn set_transformation(&mut self, transformation: Transformation) {
        self.transformation = Some(transformation);
    }

    fn normal_at(&self, point: Tuple) -> Tuple {
        let object_point = self.transformation().inverse().unwrap() * point;
        let object_normal = object_point - Tuple::origin();
        let mut world_normal = self.transformation().inverse().unwrap().transpose() * object_normal;
        world_normal.as_vector();
        world_normal.normalize()
    }

    fn material(&self) -> Material {
        self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    
}

impl From<Transformation> for Sphere {
    fn from(value: Transformation) -> Self {
        let mut sphere = Sphere::new();
        sphere.set_transformation(value);
        sphere
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersects(r);

        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0]);
        assert_eq!(6.0, xs[1]);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersects(r);

        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0]);
        assert_eq!(5.0, xs[1]);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersects(r);

        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersects(r);

        assert_eq!(2, xs.len());
        assert_eq!(-1.0, xs[0]);
        assert_eq!(1.0, xs[1]);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersects(r);

        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0]);
        assert_eq!(-4.0, xs[1]);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();

        s.set_transformation(Transformation::identity().scale(2.0, 2.0, 2.0));

        let xs = s.intersects(r);

        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0]);
        assert_eq!(7.0, xs[1]);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();

        s.set_transformation(Transformation::identity().translation(5.0, 0.0, 0.0));

        let xs = s.intersects(r);

        assert_eq!(0, xs.len());
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(1.0, 0.0, 0.0));

        assert_eq!(Tuple::vector(1.0, 0.0, 0.0), n);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(0.0, 1.0, 0.0));

        assert_eq!(Tuple::vector(0.0, 1.0, 0.0), n);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(0.0, 0.0, 1.0));

        assert_eq!(Tuple::vector(0.0, 0.0, 1.0), n);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
        ));

        assert_eq!(
            Tuple::vector(
                3.0f64.sqrt() / 3.0,
                3.0f64.sqrt() / 3.0,
                3.0f64.sqrt() / 3.0
            ),
            n
        );
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = Sphere::from(Transformation::identity().translation(0.0, 1.0, 0.0));

        let n = s.normal_at(Tuple::point(0.0, 1.70711, -0.70711));

        assert_eq!(Tuple::vector(0.0, 0.70711, -0.70711), n);
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let s = Sphere::from(
            Transformation::identity()
                .rotate_z(PI / 5.0)
                .scale(1.0, 0.5, 1.0),
        );

        let n = s.normal_at(Tuple::point(0.0, 2.0f64.sqrt() / 2.0, -2f64.sqrt() / 2.0));

        assert_eq!(Tuple::vector(0.0, 0.97014, -0.24254), n);
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::new();
        let m = s.material();

        assert_eq!(Material::new(), m);
    }

    #[test]
    fn a_spehre_may_be_assigned_a_material() {
        let mut s: Sphere = Sphere::new();
        let m = Material::new().with_ambient(1.0);
        s.set_material(m);

        assert_eq!(m, s.material());
    }
}
