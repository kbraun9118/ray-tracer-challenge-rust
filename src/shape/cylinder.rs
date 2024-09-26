use std::mem::swap;

use uuid::Uuid;

use crate::{
    intersection::{ray::Ray, Intersection, ShapeIntersection},
    transformation::Transformation,
    tuple::Tuple,
    util::{eq_f64, EPSILON},
};

use super::{material::Material, BoundedBox, Shape, WeakGroupContainer};

#[derive(Debug)]
pub struct Cylinder {
    id: uuid::Uuid,
    transformation: Transformation,
    material: Material,
    minimum: f64,
    maximum: f64,
    closed: bool,
    parent: Option<WeakGroupContainer>,
}

fn check_cap(ray: Ray, t: f64) -> bool {
    let x = ray.origin().x() + t * ray.direction().x();
    let z = ray.origin().z() + t * ray.direction().z();

    x.powi(2) + z.powi(2) <= 1.0
}

impl Cylinder {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            transformation: Transformation::default(),
            material: Material::default(),
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
            closed: false,
            parent: None,
        }
    }

    pub fn minimum(&self) -> f64 {
        self.minimum
    }

    pub fn set_minimum(&mut self, minimum: f64) {
        self.minimum = minimum
    }

    pub fn maximum(&self) -> f64 {
        self.maximum
    }

    pub fn set_maximum(&mut self, maximum: f64) {
        self.maximum = maximum
    }

    pub fn closed(&self) -> bool {
        self.closed
    }

    pub fn set_closed(&mut self, closed: bool) {
        self.closed = closed;
    }

    fn intersect_caps(&self, ray: Ray, xs: &mut Vec<Intersection>) {
        if !self.closed || eq_f64(ray.direction().y(), 0.0) {
            return;
        }

        let t = (self.minimum - ray.origin().y()) / ray.direction().y();
        if check_cap(ray, t) {
            xs.push(Intersection::new(t, self.id));
        }

        let t = (self.maximum - ray.origin().y()) / ray.direction().y();
        if check_cap(ray, t) {
            xs.push(Intersection::new(t, self.id));
        }
    }
}

impl Shape for Cylinder {
    fn id(&self) -> uuid::Uuid {
        self.id
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let a = ray.direction().x().powi(2) + ray.direction().z().powi(2);

        if eq_f64(a, 0.0) {
            let mut xs = vec![];
            self.intersect_caps(ray, &mut xs);
            return xs;
        }

        let b = 2.0 * ray.origin().x() * ray.direction().x()
            + 2.0 * ray.origin().z() * ray.direction().z();
        let c = ray.origin().x().powi(2) + ray.origin().z().powi(2) - 1.0;

        let disc = b.powi(2) - 4.0 * a * c;

        if disc < 0.0 {
            return vec![];
        }

        let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
        let mut t1 = (-b + disc.sqrt()) / (2.0 * a);

        if t0 > t1 {
            swap(&mut t0, &mut t1);
        }

        let mut xs = vec![];

        let y0 = ray.origin().y() + t0 * ray.direction().y();
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(Intersection::new(t0, self.id))
        }

        let y1 = ray.origin().y() + t1 * ray.direction().y();
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(Intersection::new(t1, self.id));
        }
        self.intersect_caps(ray, &mut xs);

        xs
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
        if self.id != id {
            return None;
        }

        let dist = point.x().powi(2) + point.z().powi(2);

        Some(if dist < 1.0 && point.y() >= self.maximum - EPSILON {
            Tuple::vector(0.0, 1.0, 0.0)
        } else if dist < 1.0 && point.y() < self.minimum + EPSILON {
            Tuple::vector(0.0, -1.0, 0.0)
        } else {
            Tuple::vector(point.x(), 0.0, point.z())
        })
    }

    fn parent(&self) -> Option<WeakGroupContainer> {
        self.parent.clone()
    }

    fn set_parent(&mut self, parent: WeakGroupContainer) {
        self.parent = Some(parent.clone());
    }

    fn bounds(&self) -> BoundedBox {
        BoundedBox::new(
            Tuple::point(-1.0, self.minimum, -1.0),
            Tuple::point(1.0, self.maximum, 1.0),
        )
    }

    fn contains(&self, id: Uuid) -> bool {
        self.id == id
    }
}

#[cfg(test)]
mod tests {
    use crate::{intersection::ray::Ray, shape::ShapeContainer, tuple::Tuple};

    use super::*;

    #[test]
    fn a_ray_misses_a_cylinder() {
        let exs = vec![
            (Tuple::point(1.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)),
            (Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)),
            (Tuple::point(0.0, 0.0, -5.0), Tuple::vector(1.0, 1.0, 1.0)),
        ];
        let cyl = Cylinder::new();

        for (origin, direction) in exs {
            let direction = direction.normalize();
            let r = Ray::new(origin, direction);
            let xs = cyl.local_intersect(r);

            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn a_ray_strikes_a_cylinder() {
        let exs = vec![
            (
                Tuple::point(1.0, 0.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Tuple::point(0.0, 0.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                Tuple::point(0.5, 0.0, -5.0),
                Tuple::vector(0.1, 1.0, 1.0),
                6.80798,
                7.08872,
            ),
        ];

        let cyl = Cylinder::new();
        for (origin, direction, t0, t1) in exs {
            let direction = direction.normalize();
            let r = Ray::new(origin, direction);
            let xs = cyl.local_intersect(r);
            assert_eq!(xs.len(), 2);
            assert!(eq_f64(xs[0].t(), t0));
            assert!(eq_f64(xs[1].t(), t1));
        }
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let exs = vec![
            (Tuple::point(1.0, 0.0, 0.0), Tuple::vector(1.0, 0.0, 0.0)),
            (Tuple::point(0.0, 5.0, -1.0), Tuple::vector(0.0, 0.0, -1.0)),
            (Tuple::point(0.0, -2.0, 1.0), Tuple::vector(0.0, 0.0, 1.0)),
            (Tuple::point(-1.0, 1.0, 0.0), Tuple::vector(-1.0, 0.0, 0.0)),
        ];

        let cyl = ShapeContainer::from(Cylinder::new());
        let i = ShapeIntersection::new(0.0, cyl.clone(), cyl.read().unwrap().id());
        for (point, normal) in exs {
            let n = cyl
                .read()
                .unwrap()
                .local_normal_at(cyl.id(), point, i.clone())
                .unwrap();
            assert_eq!(n, normal);
        }
    }

    #[test]
    fn the_default_min_and_max_for_a_cylinder() {
        let cyl = Cylinder::new();
        assert!(eq_f64(cyl.minimum(), f64::NEG_INFINITY));
        assert!(eq_f64(cyl.maximum(), f64::INFINITY));
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
        let exs = vec![
            (Tuple::point(0.0, 1.5, 0.0), Tuple::vector(0.1, 1.0, 0.0), 0),
            (
                Tuple::point(0.0, 3.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0),
                0,
            ),
            (
                Tuple::point(0.0, 0.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0),
                0,
            ),
            (
                Tuple::point(0.0, 2.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0),
                0,
            ),
            (
                Tuple::point(0.0, 1.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0),
                0,
            ),
            (
                Tuple::point(0.0, 1.5, -2.0),
                Tuple::vector(0.0, 0.0, 1.0),
                2,
            ),
        ];

        let mut cyl = Cylinder::new();
        cyl.set_minimum(1.0);
        cyl.set_maximum(2.0);

        for (point, direction, count) in exs {
            let direction = direction.normalize();
            let r = Ray::new(point, direction);
            let xs = cyl.local_intersect(r);

            assert_eq!(xs.len(), count);
        }
    }

    #[test]
    fn the_default_closed_value_for_a_cylinder() {
        let cyl = Cylinder::new();
        assert!(!cyl.closed())
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let exs = vec![
            (Tuple::point(0.0, 3.0, 0.0), Tuple::vector(0.0, -1.0, 0.0)),
            (Tuple::point(0.0, 3.0, -2.0), Tuple::vector(0.0, -1.0, 2.0)),
            (Tuple::point(0.0, 4.0, -2.0), Tuple::vector(0.0, -1.0, 1.0)),
            (Tuple::point(0.0, 0.0, -2.0), Tuple::vector(0.0, 1.0, 2.0)),
            (Tuple::point(0.0, -1.0, -2.0), Tuple::vector(0.0, 1.0, 1.0)),
        ];
        let mut cyl = Cylinder::new();
        cyl.set_minimum(1.0);
        cyl.set_maximum(2.0);
        cyl.set_closed(true);

        for (point, direction) in exs {
            let direction = direction.normalize();
            let r = Ray::new(point, direction);
            let xs = cyl.local_intersect(r);
            assert_eq!(xs.len(), 2);
        }
    }

    #[test]
    fn the_normal_vector_on_a_cylinders_end_caps() {
        let exs = vec![
            (Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0)),
            (Tuple::point(0.5, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0)),
            (Tuple::point(0.0, 1.0, 0.5), Tuple::vector(0.0, -1.0, 0.0)),
            (Tuple::point(0.0, 2.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)),
            (Tuple::point(0.5, 2.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)),
            (Tuple::point(0.0, 2.0, 0.5), Tuple::vector(0.0, 1.0, 0.0)),
        ];

        let mut cyl = Cylinder::new();
        cyl.set_minimum(1.0);
        cyl.set_maximum(2.0);
        cyl.set_closed(true);
        let cyl = ShapeContainer::from(cyl);
        let i = ShapeIntersection::new(0.0, cyl.clone(), cyl.read().unwrap().id());

        for (point, normal) in exs {
            let n = cyl
                .read()
                .unwrap()
                .local_normal_at(cyl.id(), point, i.clone())
                .unwrap();
            assert_eq!(n, normal);
        }
    }
}
