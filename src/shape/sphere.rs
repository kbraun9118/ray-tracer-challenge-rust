use crate::{
    intersection::{ray::Ray, Intersection, ShapeIntersection},
    transformation::Transformation,
    tuple::Tuple,
};
use uuid::Uuid;

use super::{group::WeakGroupContainer, material::Material, BoundedBox, Shape};

#[derive(Debug)]
pub struct Sphere {
    id: Uuid,
    center: Tuple,
    transformation: Transformation,
    material: Material,
    parent: Option<WeakGroupContainer>,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            center: Tuple::origin(),
            transformation: Transformation::identity(),
            material: Material::new(),
            parent: None,
        }
    }

    pub fn glassy() -> Self {
        Self {
            material: Material::new()
                .with_transparency(1.0)
                .with_refractive_index(1.5),
            ..Self::new()
        }
    }
}

impl Shape for Sphere {
    fn id(&self) -> Uuid {
        self.id
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin() - self.center;

        let a = ray.direction() * ray.direction();
        let b = (ray.direction() * sphere_to_ray) * 2.0;
        let c = sphere_to_ray * sphere_to_ray - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            vec![
                Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), self.id),
                Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), self.id),
            ]
        }
    }

    fn transformation(&self) -> Transformation {
        self.transformation.clone()
    }

    fn set_transformation(&mut self, transformation: Transformation) {
        self.transformation = transformation;
    }

    fn material(&self, id: Uuid) -> Option<Material> {
        if self.id == id {
            Some(self.material.clone())
        } else {
            None
        }
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_normal_at(
        &self,
        id: Uuid,
        point: Tuple,
        _intersection: ShapeIntersection,
    ) -> Option<Tuple> {
        if id == self.id {
            Some(point - Tuple::origin())
        } else {
            None
        }
    }

    fn parent(&self) -> Option<WeakGroupContainer> {
        self.parent.clone()
    }

    fn set_parent(&mut self, parent: WeakGroupContainer) {
        self.parent = Some(parent);
    }

    fn bounds(&self) -> BoundedBox {
        BoundedBox::new(Tuple::point(-1.0, -1.0, -1.0), Tuple::point(1.0, 1.0, 1.0))
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

    use crate::shape::ShapeContainer;

    use super::*;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersects(r);

        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t());
        assert_eq!(6.0, xs[1].t());
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersects(r);

        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0].t());
        assert_eq!(5.0, xs[1].t());
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
        assert_eq!(-1.0, xs[0].t());
        assert_eq!(1.0, xs[1].t());
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersects(r);

        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0].t());
        assert_eq!(-4.0, xs[1].t());
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();

        s.set_transformation(Transformation::identity().scale(2.0, 2.0, 2.0));

        let xs = s.intersects(r);

        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0].t());
        assert_eq!(7.0, xs[1].t());
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
        let s = ShapeContainer::from(Sphere::new());
        let i = ShapeIntersection::new(0.0, s.clone(), s.read().unwrap().id());
        let n = s
            .read()
            .unwrap()
            .normal_at(s.read().unwrap().id(), Tuple::point(1.0, 0.0, 0.0), i)
            .unwrap();

        assert_eq!(Tuple::vector(1.0, 0.0, 0.0), n);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = ShapeContainer::from(Sphere::new());
        let i = ShapeIntersection::new(0.0, s.clone(), s.read().unwrap().id());
        let n = s
            .read()
            .unwrap()
            .normal_at(s.read().unwrap().id(), Tuple::point(0.0, 1.0, 0.0), i)
            .unwrap();

        assert_eq!(Tuple::vector(0.0, 1.0, 0.0), n);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = ShapeContainer::from(Sphere::new());
        let i = ShapeIntersection::new(0.0, s.clone(), s.read().unwrap().id());
        let n = s
            .read()
            .unwrap()
            .normal_at(s.read().unwrap().id(), Tuple::point(0.0, 0.0, 1.0), i)
            .unwrap();

        assert_eq!(Tuple::vector(0.0, 0.0, 1.0), n);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = ShapeContainer::from(Sphere::new());
        let i = ShapeIntersection::new(0.0, s.clone(), s.read().unwrap().id());
        let n = s
            .read()
            .unwrap()
            .normal_at(
                s.id(),
                Tuple::point(
                    3.0f64.sqrt() / 3.0,
                    3.0f64.sqrt() / 3.0,
                    3.0f64.sqrt() / 3.0,
                ),
                i,
            )
            .unwrap();

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
        let s = ShapeContainer::from(Sphere::from(
            Transformation::identity().translation(0.0, 1.0, 0.0),
        ));
        let i = ShapeIntersection::new(0.0, s.clone(), s.read().unwrap().id());

        let n = s
            .read()
            .unwrap()
            .normal_at(s.id(), Tuple::point(0.0, 1.70711, -0.70711), i)
            .unwrap();

        assert_eq!(Tuple::vector(0.0, 0.70711, -0.70711), n);
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let s = ShapeContainer::from(Sphere::from(
            Transformation::identity()
                .rotate_z(PI / 5.0)
                .scale(1.0, 0.5, 1.0),
        ));
        let i = ShapeIntersection::new(0.0, s.clone(), s.read().unwrap().id());

        let n = s
            .read()
            .unwrap()
            .normal_at(
                s.id(),
                Tuple::point(0.0, 2.0f64.sqrt() / 2.0, -2f64.sqrt() / 2.0),
                i,
            )
            .unwrap();

        assert_eq!(Tuple::vector(0.0, 0.97014, -0.24254), n);
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::new();
        let m = s.material(s.id());

        assert_eq!(Material::new(), m.unwrap());
    }

    #[test]
    fn a_spehre_may_be_assigned_a_material() {
        let mut s: Sphere = Sphere::new();
        let m = Material::new().with_ambient(1.0);
        s.set_material(m.clone());

        assert_eq!(m, s.material(s.id()).unwrap());
    }

    #[test]
    fn a_helper_for_producing_a_sphere_with_a_glassy_material() {
        let s = Sphere::glassy();
        assert_eq!(Transformation::identity(), s.transformation());
        assert_eq!(1.0, s.material(s.id()).unwrap().transparency());
        assert_eq!(1.5, s.material(s.id()).unwrap().refractive_index());
    }
}
