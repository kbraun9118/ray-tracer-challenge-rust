use uuid::Uuid;

use crate::{
    intersection::{ray::Ray, Intersection},
    transformation::Transformation,
    tuple::Tuple,
    util,
};

use super::{bounded_box::BoundedBox, group::WeakGroupContainer, material::Material, Shape};

#[derive(Debug, Clone)]
pub struct Triangle {
    id: Uuid,
    transformation: Transformation,
    material: Material,
    parent: Option<WeakGroupContainer>,
    p1: Tuple,
    p2: Tuple,
    p3: Tuple,
    e1: Tuple,
    e2: Tuple,
    normal: Tuple,
}

impl Triangle {
    pub fn new(p1: Tuple, p2: Tuple, p3: Tuple) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        Self {
            id: Uuid::new_v4(),
            transformation: Transformation::identity(),
            material: Material::new(),
            parent: None,
            p1,
            p2,
            p3,
            e1,
            e2,
            normal: (e2 ^ e1).normalize(),
        }
    }

    #[allow(unused)]
    pub(crate) fn p1(&self) -> Tuple {
        self.p1
    }

    #[allow(unused)]
    pub(crate) fn p2(&self) -> Tuple {
        self.p2
    }

    #[allow(unused)]
    pub(crate) fn p3(&self) -> Tuple {
        self.p3
    }
}

impl Shape for Triangle {
    fn id(&self) -> uuid::Uuid {
        self.id
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let dir_cross_e2 = ray.direction() ^ self.e2;
        let det = self.e1 * dir_cross_e2;

        if det.abs() < util::EPSILON {
            return vec![];
        }

        let f = 1.0 / det;
        let p1_to_origin = ray.origin() - self.p1;
        let u = f * (p1_to_origin * dir_cross_e2);

        if u < 0.0 || u > 1.0 {
            return vec![];
        }

        let origin_cross_e1 = p1_to_origin ^ self.e1;
        let v = f * (ray.direction() * origin_cross_e1);

        if v < 0.0 || u + v > 1.0 {
            return vec![];
        }

        vec![Intersection::new(f * (self.e2 * origin_cross_e1), self.id)]
    }

    fn transformation(&self) -> Transformation {
        self.transformation.clone()
    }

    fn set_transformation(&mut self, transformation: Transformation) {
        self.transformation = transformation;
    }

    fn material(&self, id: uuid::Uuid) -> Option<Material> {
        if self.id == id {
            Some(self.material.clone())
        } else {
            None
        }
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_normal_at(&self, id: uuid::Uuid, _point: Tuple) -> Option<Tuple> {
        if self.id == id {
            Some(self.normal)
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
        let mut bbox = BoundedBox::empty();
        bbox.add_point(self.p1);
        bbox.add_point(self.p2);
        bbox.add_point(self.p3);
        bbox
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_triangle() -> Triangle {
        let p1 = Tuple::point(0.0, 1.0, 0.0);
        let p2 = Tuple::point(-1.0, 0.0, 0.0);
        let p3 = Tuple::point(1.0, 0.0, 0.0);
        Triangle::new(p1, p2, p3)
    }

    #[test]
    fn constructing_a_triangle() {
        let t = test_triangle();

        assert_eq!(t.p1, Tuple::point(0.0, 1.0, 0.0));
        assert_eq!(t.p2, Tuple::point(-1.0, 0.0, 0.0));
        assert_eq!(t.p3, Tuple::point(1.0, 0.0, 0.0));
        assert_eq!(t.e1, Tuple::vector(-1.0, -1.0, 0.0));
        assert_eq!(t.e2, Tuple::vector(1.0, -1.0, 0.0));
        assert_eq!(t.normal, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn finding_the_normal_on_a_triangle() {
        let t = test_triangle();
        let n1 = t.normal_at(t.id(), Tuple::point(0.0, 0.5, 0.0)).unwrap();
        let n2 = t.normal_at(t.id(), Tuple::point(-0.5, 0.75, 0.0)).unwrap();
        let n3 = t.normal_at(t.id(), Tuple::point(0.5, 0.25, 0.0)).unwrap();

        assert_eq!(n1, t.normal);
        assert_eq!(n2, t.normal);
        assert_eq!(n3, t.normal);
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let t = test_triangle();
        let r = Ray::new(Tuple::point(0.0, -1.0, -2.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = t.local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p1_p3_edge() {
        let t = test_triangle();
        let r = Ray::new(Tuple::point(1.0, 1.0, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = t.local_intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p1_p2_edge() {
        let t = test_triangle();
        let r = Ray::new(Tuple::point(-1.0, 1.0, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = t.local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p2_p3_edge() {
        let t = test_triangle();
        let r = Ray::new(Tuple::point(0.0, -1.0, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = t.local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_strikes_the_triangle() {
        let t = test_triangle();
        let r = Ray::new(Tuple::point(0.0, 0.5, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = t.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t(), 2.0);
    }
}
