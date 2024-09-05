use std::{f64::INFINITY, mem::swap};

use crate::{intersection::ray::Ray, transformation::Transformation, tuple::Tuple, util};

use super::{material::Material, Shape};

#[derive(Debug)]
pub struct Cube {
    id: uuid::Uuid,
    transformation: Transformation,
    material: Material,
}

impl Cube {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            transformation: Transformation::default(),
            material: Material::default(),
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

    fn local_intersect(&self, ray: Ray) -> Vec<f64> {
        let (xtmin, xtmax) = dbg!(check_axis(ray.origin().x(), ray.direction().x()));
        let (ytmin, ytmax) = dbg!(check_axis(ray.origin().y(), ray.direction().y()));
        let (ztmin, ztmax) = dbg!(check_axis(ray.origin().z(), ray.direction().z()));

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        vec![tmin, tmax]
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

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::tuple::Tuple;

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

        for (origin, direction, t1, t2) in input {
            let c = Cube::new();
            let r = Ray::new(origin, direction);
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0], t1);
            assert_eq!(xs[1], t2);
        }

    }
}
