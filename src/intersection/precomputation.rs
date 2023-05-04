use std::rc::Rc;

use crate::{
    intersection::{Intersection, ray::Ray},
    shape::Shape,
    tuple::Tuple,
    util::EPSILON,
};

use super::IntersectionHeap;

#[derive(Debug, Clone)]
pub struct PreComputations {
    t: f64,
    object: Rc<dyn Shape>,
    point: Tuple,
    over_point: Tuple,
    eye_v: Tuple,
    normal_v: Tuple,
    reflect_v: Tuple,
    n1: f64,
    n2: f64,
    inside: bool,
}

impl PreComputations {
    pub fn new(intersection: Intersection, ray: Ray, xs: &IntersectionHeap) -> Self {
        let point = ray.position(intersection.t());
        let mut normal_v = intersection.object().normal_at(point);
        let eye_v = -ray.direction();
        let mut inside = false;

        if normal_v * eye_v < 0.0 {
            inside = true;
            normal_v = -normal_v
        }

        let (mut n1, mut n2) = (0.0, 0.0);

        let mut containers: Vec<Rc<dyn Shape>> = vec![];

        for i in xs.iter() {
            if let Some(hit) = xs.hit() {
                if i == &hit {
                    if let Some(last) = containers.last() {
                        n1 = last.material().refractive_index()
                    } else {
                        n1 = 1.0
                    }
                }
            }

            if containers.iter().any(|c| c.id() == i.object().id()) {
                containers.retain(|c| c.id() != i.object().id());
            } else {
                containers.push(i.object().clone());
            }

            if let Some(hit) = xs.hit() {
                if i == &hit {
                    if let Some(last) = containers.last() {
                        n2 = last.material().refractive_index()
                    } else {
                        n2 = 1.0
                    }
                    break;
                }
            }
        }

        dbg!((n1, n2));

        Self {
            t: intersection.t(),
            object: intersection.object().clone(),
            point,
            over_point: point + normal_v * EPSILON,
            eye_v,
            normal_v,
            reflect_v: ray.direction().reflect(normal_v),
            n1,
            n2,
            inside,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &Rc<dyn Shape> {
        &self.object
    }

    pub fn point(&self) -> Tuple {
        self.point
    }

    pub fn over_point(&self) -> Tuple {
        self.over_point
    }

    pub fn eye_v(&self) -> Tuple {
        self.eye_v
    }

    pub fn normal_v(&self) -> Tuple {
        self.normal_v
    }

    pub fn reflect_v(&self) -> Tuple {
        self.reflect_v
    }

    pub fn n1(&self) -> f64 {
        self.n1
    }

    pub fn n2(&self) -> f64 {
        self.n2
    }

    pub fn inside(&self) -> bool {
        self.inside
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        shape::{material::Material, plane::Plane, sphere::Sphere},
        transformation::Transformation,
    };

    use super::*;

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Rc::new(Sphere::new());
        let i = Intersection::new(4.0, s.clone());

        let comps = PreComputations::new(i.clone(), r, &mut IntersectionHeap::new());

        assert_eq!(i.t(), comps.t());
        assert_eq!(i.object().as_ref(), comps.object().as_ref());
        assert_eq!(Tuple::point(0.0, 0.0, -1.0), comps.point());
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), comps.eye_v());
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), comps.normal_v());
        assert_eq!(false, comps.inside());
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Rc::new(Sphere::new());
        let i = Intersection::new(1.0, s.clone());

        let comps = PreComputations::new(i.clone(), r, &mut IntersectionHeap::new());

        assert_eq!(i.t(), comps.t());
        assert_eq!(i.object().as_ref(), comps.object().as_ref());
        assert_eq!(Tuple::point(0.0, 0.0, 1.0), comps.point());
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), comps.eye_v());
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), comps.normal_v());
        assert_eq!(true, comps.inside());
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transformation(Transformation::identity().translation(0.0, 0.0, 1.0));

        let i = Intersection::new(5.0, Rc::new(s));
        let comps = PreComputations::new(i.clone(), r, &mut IntersectionHeap::new());

        assert!(comps.over_point().z() < -EPSILON / 2.0);
        assert!(comps.point().z() > comps.over_point().z());
    }

    #[test]
    fn pre_computing_the_reflection_vector() {
        let shape = Rc::new(Plane::new());
        let r = Ray::new(
            Tuple::point(0.0, 1.0, -1.0),
            Tuple::vector(0.0, -(2f64.sqrt()) / 2.0, 2f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2f64.sqrt(), shape.clone());

        let comps = PreComputations::new(i, r, &mut IntersectionHeap::new());

        assert_eq!(
            Tuple::vector(0.0, 2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0),
            comps.reflect_v()
        );
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut a = Sphere::glassy();
        a.set_transformation(Transformation::identity().scale(2.0, 2.0, 2.0));
        a.set_material(Material::new().with_refractive_index(1.5));
        let a = Rc::new(a);

        let mut b = Sphere::glassy();
        b.set_transformation(Transformation::identity().translation(0.0, 0.0, -0.25));
        b.set_material(Material::new().with_refractive_index(2.0));
        let b = Rc::new(b);

        let mut c = Sphere::glassy();
        c.set_transformation(Transformation::identity().translation(0.0, 0.0, 0.25));
        c.set_material(Material::new().with_refractive_index(2.5));
        let c = Rc::new(c);

        let r = Ray::new(Tuple::point(0.0, 0.0, -4.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut xs = vec![
            (2.0, a.clone()),
            (2.75, b.clone()),
            (3.25, c.clone()),
            (4.75, b.clone()),
            (5.25, c.clone()),
            (6.0, a.clone()),
        ]
            .into_iter()
            .map(|(t, obj)| Intersection::new(t, obj))
            .collect::<IntersectionHeap>();

        let ns = vec![
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];

        for (i, (n1, n2)) in ns.into_iter().enumerate() {
            dbg!(i);
            let intersection = xs[i].clone();
            dbg!(xs[i].t());
            let comps = PreComputations::new(intersection, r, &mut xs);
            assert_eq!(n1, comps.n1());
            assert_eq!(n2, comps.n2());
        }
    }
}
