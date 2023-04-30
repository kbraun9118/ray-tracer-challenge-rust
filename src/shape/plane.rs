use uuid::Uuid;

use crate::{intersection::ray::Ray, transformation::Transformation, tuple::Tuple, util::EPSILON};

use super::{material::Material, Shape};

#[derive(Debug)]
pub struct Plane {
    id: Uuid,
    material: Material,
    transformation: Transformation,
}

impl Plane {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            material: Material::new(),
            transformation: Transformation::identity(),
        }
    }
}

impl Shape for Plane {
    fn id(&self) -> Uuid {
        self.id
    }

    fn local_intersect(&self, ray: Ray) -> Vec<f64> {
        if ray.direction().y().abs() < EPSILON {
            vec![]
        } else {
            vec![-ray.origin().y() / ray.direction().y()]
        }
    }

    fn transformation(&self) -> Transformation {
        self.transformation.clone()
    }

    fn set_transformation(&mut self, transformation: Transformation) {
        self.transformation = transformation;
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_normal_at(&self, _point: Tuple) -> Tuple {
        Tuple::vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();
        let n1 = p.local_normal_at(Tuple::point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(Tuple::point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(Tuple::point(-5.0, 0.0, 150.0));

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
        assert_eq!(xs[0], 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 1.0);
    }
}
