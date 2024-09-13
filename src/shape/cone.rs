use std::mem::swap;

use uuid::Uuid;

use crate::{
    intersection::{ray::Ray, Intersection},
    transformation::Transformation,
    tuple::Tuple,
    util::{eq_f64, EPSILON},
};

use super::{group::WeakGroupContainer, material::Material, BoundedBox, Shape};

#[derive(Debug)]
pub struct Cone {
    id: uuid::Uuid,
    transformation: Transformation,
    material: Material,
    minimum: f64,
    maximum: f64,
    closed: bool,
    parent: Option<WeakGroupContainer>,
}

fn check_cap(ray: Ray, t: f64, y: f64) -> bool {
    let x = ray.origin().x() + t * ray.direction().x();
    let z = ray.origin().z() + t * ray.direction().z();

    x.powi(2) + z.powi(2) <= y.powi(2)
}

impl Cone {
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
        if check_cap(ray, t, self.minimum) {
            xs.push(Intersection::new(t, self.id));
        }

        let t = (self.maximum - ray.origin().y()) / ray.direction().y();
        if check_cap(ray, t, self.maximum) {
            xs.push(Intersection::new(t, self.id));
        }
    }
}

impl Shape for Cone {
    fn id(&self) -> uuid::Uuid {
        self.id
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let a =
            ray.direction().x().powi(2) - ray.direction().y().powi(2) + ray.direction().z().powi(2);

        let b = 2.0 * ray.origin().x() * ray.direction().x()
            - 2.0 * ray.origin().y() * ray.direction().y()
            + 2.0 * ray.origin().z() * ray.direction().z();
        let c = ray.origin().x().powi(2) - ray.origin().y().powi(2) + ray.origin().z().powi(2);

        let a0 = eq_f64(a, 0.0);
        let b0 = eq_f64(b, 0.0);

        let mut xs = vec![];

        if a0 && b0 {
            self.intersect_caps(ray, &mut xs);
            return xs;
        } else if a0 {
            let t = -c / (2.0 * b);
            xs.push(Intersection::new(t, self.id));
            self.intersect_caps(ray, &mut xs);
            return xs;
        }

        let disc = b.powi(2) - 4.0 * a * c;

        if disc < 0.0 {
            return vec![];
        }

        let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
        let mut t1 = (-b + disc.sqrt()) / (2.0 * a);

        if t0 > t1 {
            swap(&mut t0, &mut t1);
        }

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

    fn local_normal_at(&self, id: Uuid, point: Tuple) -> Option<Tuple> {
        if self.id != id {
            return None;
        }

        let dist = point.x().powi(2) + point.z().powi(2);

        Some(if dist < 1.0 && point.y() >= self.maximum - EPSILON {
            Tuple::vector(0.0, 1.0, 0.0)
        } else if dist < 1.0 && point.y() < self.minimum + EPSILON {
            Tuple::vector(0.0, -1.0, 0.0)
        } else {
            let mut y = (point.x().powi(2) + point.z().powi(2)).sqrt();
            if point.y() > 0.0 {
                y = -y;
            }
            Tuple::vector(point.x(), y, point.z())
        })
    }

    fn parent(&self) -> Option<WeakGroupContainer> {
        self.parent.clone()
    }

    fn set_parent(&mut self, parent: WeakGroupContainer) {
        self.parent = Some(parent.clone())
    }

    fn bounds(&self) -> BoundedBox {
        let a = self.minimum.abs();
        let b = self.maximum.abs();
        let limit = if a.is_infinite() || b.is_infinite() {
            f64::INFINITY
        } else {
            a.max(b)
        };
        BoundedBox::new(
            Tuple::point(-limit, self.minimum, -limit),
            Tuple::point(limit, self.maximum, limit),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn intersecting_a_cone_with_a_ray() {
        let exs = vec![
            (
                Tuple::point(0.0, 0.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Tuple::point(0.0, 0.0, -5.0),
                Tuple::vector(1.0, 1.0, 1.0),
                8.66025,
                8.66025,
            ),
            (
                Tuple::point(1.0, 1.0, -5.0),
                Tuple::vector(-0.5, -1.0, 1.0),
                4.55006,
                49.44994,
            ),
        ];

        let shape = Cone::new();

        for (origin, direciton, t0, t1) in exs {
            let direction = direciton.normalize();
            let r = Ray::new(origin, direction);
            let xs = shape.local_intersect(r);

            assert_eq!(xs.len(), 2);
            assert!(eq_f64(xs[0].t(), t0));
            assert!(eq_f64(xs[1].t(), t1));
        }
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let shape = Cone::new();
        let direction = Tuple::vector(0.0, 1.0, 1.0).normalize();
        let r = Ray::new(Tuple::point(0.0, 0.0, -1.0), direction);
        let xs = shape.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert!(eq_f64(xs[0].t(), 0.35355));
    }

    #[test]
    fn intersecting_a_cones_end_caps() {
        let exs = vec![
            (
                Tuple::point(0.0, 0.0, -5.0),
                Tuple::vector(0.0, 1.0, 0.0),
                0,
            ),
            (
                Tuple::point(0.0, 0.0, -0.25),
                Tuple::vector(0.0, 1.0, 1.0),
                2,
            ),
            (
                Tuple::point(0.0, 0.0, -0.25),
                Tuple::vector(0.0, 1.0, 0.0),
                4,
            ),
        ];

        let mut shape = Cone::new();
        shape.set_minimum(-0.5);
        shape.set_maximum(0.5);
        shape.set_closed(true);

        for (origin, direction, count) in exs {
            let direction = direction.normalize();
            let r = Ray::new(origin, direction);
            let xs = shape.local_intersect(r);

            assert_eq!(xs.len(), count);
        }
    }

    #[test]
    fn computing_the_normal_vector_on_a_cone() {
        let exs = vec![
            (Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 0.0)),
            (
                Tuple::point(1.0, 1.0, 1.0),
                Tuple::vector(1.0, -(2f64.sqrt()), 1.0),
            ),
            (Tuple::point(-1.0, -1.0, 0.0), Tuple::vector(-1.0, 1.0, 0.0)),
        ];
        let shape = Cone::new();

        for (point, normal) in exs {
            let n = shape.local_normal_at(shape.id(), point).unwrap();
            assert_eq!(n, normal);
        }
    }
}
