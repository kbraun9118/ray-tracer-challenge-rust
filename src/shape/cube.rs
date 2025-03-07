use std::{f64::INFINITY, mem::swap};

use uuid::Uuid;

use crate::{
    intersection::{ray::Ray, Intersection, ShapeIntersection},
    transformation::Transformation,
    tuple::Tuple,
    util::{self, eq_f64},
};

use super::{material::Material, BoundedBox, Shape, WeakGroupContainer};

#[derive(Debug)]
pub struct Cube {
    id: uuid::Uuid,
    transformation: Transformation,
    material: Material,
    parent: Option<WeakGroupContainer>,
}

impl Cube {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            transformation: Transformation::default(),
            material: Material::default(),
            parent: None,
        }
    }
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

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

impl Shape for Cube {
    fn id(&self) -> uuid::Uuid {
        self.id
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let (xtmin, xtmax) = check_axis(ray.origin().x(), ray.direction().x());
        let (ytmin, ytmax) = check_axis(ray.origin().y(), ray.direction().y());
        let (ztmin, ztmax) = check_axis(ray.origin().z(), ray.direction().z());

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin > tmax {
            vec![]
        } else {
            vec![
                Intersection::new(tmin, self.id),
                Intersection::new(tmax, self.id),
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
        if self.id != id {
            return None;
        }

        let max_c = point.x().abs().max(point.y().abs()).max(point.z().abs());

        Some(if eq_f64(max_c, point.x().abs()) {
            Tuple::vector(point.x(), 0.0, 0.0)
        } else if eq_f64(max_c, point.y().abs()) {
            Tuple::vector(0.0, point.y(), 0.0)
        } else {
            Tuple::vector(0.0, 0.0, point.z())
        })
    }

    fn parent(&self) -> Option<WeakGroupContainer> {
        self.parent.clone()
    }

    fn set_parent(&mut self, parent: WeakGroupContainer) {
        self.parent = Some(parent.clone());
    }

    fn bounds(&self) -> BoundedBox {
        BoundedBox::new(Tuple::point(-1.0, -1.0, -1.0), Tuple::point(1.0, 1.0, 1.0))
    }

    fn contains(&self, id: Uuid) -> bool {
        self.id == id
    }
}

#[cfg(test)]
mod tests {
    use crate::{shape::ShapeContainer, tuple::Tuple};

    use super::*;

    #[test]
    fn a_ray_intersects_a_cube() {
        let input = vec![
            (
                Tuple::point(5.0, 0.5, 0.0),
                Tuple::vector(-1.0, 0.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Tuple::point(-5.0, 0.5, 0.0),
                Tuple::vector(1.0, 0.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Tuple::point(0.5, 5.0, 0.0),
                Tuple::vector(0.0, -1.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Tuple::point(0.5, -5.0, 0.0),
                Tuple::vector(0.0, 1.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Tuple::point(0.5, 0.0, 5.0),
                Tuple::vector(0.0, 0.0, -1.0),
                4.0,
                6.0,
            ),
            (
                Tuple::point(0.5, 0.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                Tuple::point(0.0, 0.5, 0.0),
                Tuple::vector(0.0, 0.0, 1.0),
                -1.0,
                1.0,
            ),
        ];

        let c = Cube::new();
        for (origin, direction, t1, t2) in input {
            let r = Ray::new(origin, direction);
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t(), t1);
            assert_eq!(xs[1].t(), t2);
        }
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let input = vec![
            (
                Tuple::point(-2.0, 0.0, 0.0),
                Tuple::vector(0.2673, 0.5345, 0.8018),
            ),
            (
                Tuple::point(0.0, -2.0, 0.0),
                Tuple::vector(0.8018, 0.2673, 0.5345),
            ),
            (
                Tuple::point(0.0, 0.0, -2.0),
                Tuple::vector(0.5345, 0.8018, 0.2673),
            ),
            (Tuple::point(2.0, 0.0, 2.0), Tuple::vector(0.0, 0.0, -1.0)),
            (Tuple::point(0.0, 2.0, 2.0), Tuple::vector(0.0, -1.0, 0.0)),
            (Tuple::point(2.0, 2.0, 0.0), Tuple::vector(-1.0, 0.0, 0.0)),
        ];

        let c = Cube::new();
        for (origin, direction) in input {
            let r = Ray::new(origin, direction);
            let xs = c.local_intersect(r);
            assert!(xs.is_empty());
        }
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let input = vec![
            (Tuple::point(1.0, 0.5, -0.8), Tuple::vector(1.0, 0.0, 0.0)),
            (Tuple::point(-1.0, -0.2, 0.9), Tuple::vector(-1.0, 0.0, 0.0)),
            (Tuple::point(-0.4, 1.0, -0.1), Tuple::vector(0.0, 1.0, 0.0)),
            (Tuple::point(0.3, -1.0, -0.7), Tuple::vector(0.0, -1.0, 0.0)),
            (Tuple::point(-0.6, 0.3, 1.0), Tuple::vector(0.0, 0.0, 1.0)),
            (Tuple::point(0.4, 0.4, -1.0), Tuple::vector(0.0, 0.0, -1.0)),
            (Tuple::point(1.0, 1.0, 1.0), Tuple::vector(1.0, 0.0, 0.0)),
            (
                Tuple::point(-1.0, -1.0, -1.0),
                Tuple::vector(-1.0, 0.0, 0.0),
            ),
        ];
        let c = ShapeContainer::from(Cube::new());
        let i = ShapeIntersection::new(0.0, c.clone(), c.read().unwrap().id());

        for (point, normal) in input {
            let n = c
                .read()
                .unwrap()
                .local_normal_at(c.read().unwrap().id(), point, i.clone())
                .unwrap();
            assert_eq!(n, normal);
        }
    }
}
