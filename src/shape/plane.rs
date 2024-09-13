use core::f64;

use uuid::Uuid;

use crate::{
    intersection::{ray::Ray, Intersection},
    transformation::Transformation,
    tuple::Tuple,
    util::EPSILON,
};

use super::{group::WeakGroupContainer, material::Material, BoundedBox, Shape};

#[derive(Debug)]
pub struct Plane {
    id: Uuid,
    material: Material,
    transformation: Transformation,
    parent: Option<WeakGroupContainer>,
}

impl Plane {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            material: Material::new(),
            transformation: Transformation::identity(),
            parent: None,
        }
    }
}

impl Shape for Plane {
    fn id(&self) -> Uuid {
        self.id
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        if ray.direction().y().abs() < EPSILON {
            vec![]
        } else {
            vec![Intersection::new(
                -ray.origin().y() / ray.direction().y(),
                self.id,
            )]
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

    fn local_normal_at(&self, id: Uuid, _point: Tuple) -> Option<Tuple> {
        if self.id == id {
            Some(Tuple::vector(0.0, 1.0, 0.0))
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
        BoundedBox::new(
            Tuple::point(f64::NEG_INFINITY, 0.0, f64::NEG_INFINITY),
            Tuple::point(f64::INFINITY, 0.0, f64::INFINITY),
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();
        let n1 = p
            .local_normal_at(p.id(), Tuple::point(0.0, 0.0, 0.0))
            .unwrap();
        let n2 = p
            .local_normal_at(p.id(), Tuple::point(10.0, 0.0, -10.0))
            .unwrap();
        let n3 = p
            .local_normal_at(p.id(), Tuple::point(-5.0, 0.0, 150.0))
            .unwrap();

        assert_eq!(n1, Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(n2, Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(n3, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_plane() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0.0, 10.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_a_ray_coplanar_with_the_plane() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));
        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t(), 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t(), 1.0);
    }
}
