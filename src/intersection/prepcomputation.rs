use uuid::Uuid;

use crate::{intersection::ray::Ray, shape::ShapeContainer, tuple::Tuple, util::EPSILON};

use super::{IntersectionHeap, ShapeIntersection};

#[derive(Debug, Clone)]
pub struct PrepComputations {
    t: f64,
    object: ShapeContainer,
    object_id: Uuid,
    point: Tuple,
    over_point: Tuple,
    under_point: Tuple,
    eye_v: Tuple,
    normal_v: Tuple,
    reflect_v: Tuple,
    n1: f64,
    n2: f64,
    inside: bool,
}

impl PrepComputations {
    pub fn new(intersection: ShapeIntersection, ray: Ray, xs: &IntersectionHeap) -> Self {
        let point = ray.position(intersection.t());
        let mut normal_v = intersection
            .object()
            .borrow()
            .normal_at(intersection.object_id(), point)
            .unwrap();
        let eye_v = -ray.direction();
        let mut inside = false;

        if normal_v * eye_v < 0.0 {
            inside = true;
            normal_v = -normal_v
        }

        let (mut n1, mut n2) = (0.0, 0.0);

        let mut containers: Vec<(ShapeContainer, Uuid)> = vec![];

        for i in xs.iter() {
            if i == &intersection {
                if let Some((last, last_id)) = containers.last() {
                    n1 = last.borrow().material(*last_id).unwrap().refractive_index()
                } else {
                    n1 = 1.0
                }
            }

            if containers
                .iter()
                .any(|(c, _)| c.borrow().id() == i.object().borrow().id())
            {
                containers.retain(|(c, _)| c.borrow().id() != i.object().borrow().id());
            } else {
                containers.push((i.object().clone(), i.object_id()));
            }

            if i == &intersection {
                if let Some((last, last_id)) = containers.last() {
                    n2 = last.borrow().material(*last_id).unwrap().refractive_index()
                } else {
                    n2 = 1.0
                }
                break;
            }
        }

        Self {
            t: intersection.t(),
            object: intersection.object().clone(),
            object_id: intersection.object_id,
            point,
            over_point: point + normal_v * EPSILON,
            under_point: point - normal_v * EPSILON,
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

    pub fn object(&self) -> ShapeContainer {
        self.object.clone()
    }

    pub fn object_id(&self) -> Uuid {
        self.object_id
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

    pub fn under_point(&self) -> Tuple {
        self.under_point
    }

    pub fn schlick(&self) -> f64 {
        let mut cos = self.eye_v() * self.normal_v();

        if self.n1() > self.n2() {
            let n = self.n1() / self.n2();
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }

            let cos_t = (1.0 - sin2_t).sqrt();
            cos = cos_t;
        }
        let r0 = ((self.n1() - self.n2()) / (self.n1() + self.n2())).powi(2);

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        intersections,
        shape::{material::Material, plane::Plane, sphere::Sphere, Shape},
        transformation::Transformation,
        util::eq_f64,
    };

    use super::*;

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = ShapeContainer::from(Sphere::new());
        let i = ShapeIntersection::new(4.0, s.clone(), s.id());

        let comps = PrepComputations::new(i.clone(), r, &mut IntersectionHeap::new());

        assert_eq!(i.t(), comps.t());
        assert_eq!(i.object().borrow().id(), comps.object().borrow().id());
        assert_eq!(Tuple::point(0.0, 0.0, -1.0), comps.point());
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), comps.eye_v());
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), comps.normal_v());
        assert_eq!(false, comps.inside());
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = ShapeContainer::from(Sphere::new());
        let i = ShapeIntersection::new(1.0, s.clone(), s.id());

        let comps = PrepComputations::new(i.clone(), r, &mut IntersectionHeap::new());

        assert_eq!(i.t(), comps.t());
        assert_eq!(i.object().borrow().id(), comps.object().borrow().id());
        assert_eq!(Tuple::point(0.0, 0.0, 1.0), comps.point());
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), comps.eye_v());
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), comps.normal_v());
        assert_eq!(true, comps.inside());
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        let s_id = s.id();
        s.set_transformation(Transformation::identity().translation(0.0, 0.0, 1.0));

        let i = ShapeIntersection::new(5.0, ShapeContainer::from(s), s_id);
        let comps = PrepComputations::new(i.clone(), r, &mut IntersectionHeap::new());

        assert!(comps.over_point().z() < -EPSILON / 2.0);
        assert!(comps.point().z() > comps.over_point().z());
    }

    #[test]
    fn pre_computing_the_reflection_vector() {
        let shape = ShapeContainer::from(Plane::new());
        let r = Ray::new(
            Tuple::point(0.0, 1.0, -1.0),
            Tuple::vector(0.0, -(2f64.sqrt()) / 2.0, 2f64.sqrt() / 2.0),
        );
        let i = ShapeIntersection::new(2f64.sqrt(), shape.clone(), shape.id());

        let comps = PrepComputations::new(i, r, &mut IntersectionHeap::new());

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
        let a = ShapeContainer::from(a);

        let mut b = Sphere::glassy();
        b.set_transformation(Transformation::identity().translation(0.0, 0.0, -0.25));
        b.set_material(Material::new().with_refractive_index(2.0));
        let b = ShapeContainer::from(b);

        let mut c = Sphere::glassy();
        c.set_transformation(Transformation::identity().translation(0.0, 0.0, 0.25));
        c.set_material(Material::new().with_refractive_index(2.5));
        let c = ShapeContainer::from(c);

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
        .map(|(t, obj)| ShapeIntersection::new(t, obj.clone(), obj.borrow().id()))
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
            let intersection = xs[i].clone();
            let comps = PrepComputations::new(intersection, r, &mut xs);
            assert_eq!(n1, comps.n1());
            assert_eq!(n2, comps.n2());
        }
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut shape = Sphere::glassy();
        shape.set_transformation(Transformation::identity().translation(0.0, 0.0, 1.0));
        let shape = ShapeContainer::from(shape);

        let i = ShapeIntersection::new(5.0, shape.clone(), shape.id());
        let xs = intersections!(i.clone());
        let comps = PrepComputations::new(i, r, &xs);

        assert!(comps.under_point().z() > EPSILON / 2.0);
        assert!(comps.point().z() < comps.under_point().z());
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let shape = ShapeContainer::from(Sphere::glassy());
        let r = Ray::new(
            Tuple::point(0.0, 0.0, 2f64.sqrt() / 2.0),
            Tuple::vector(0.0, 1.0, 0.0),
        );
        let xs = intersections!(
            ShapeIntersection::new(-(2f64.sqrt()) / 2.0, shape.clone(), shape.id()),
            ShapeIntersection::new(2f64.sqrt() / 2.0, shape.clone(), shape.id())
        );
        let comps = PrepComputations::new(xs[1].clone(), r, &xs);
        let reflectance = comps.schlick();
        assert!(eq_f64(reflectance, 1.0));
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = ShapeContainer::from(Sphere::glassy());
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = intersections!(
            ShapeIntersection::new(-1.0, shape.clone(), shape.id()),
            ShapeIntersection::new(1.0, shape.clone(), shape.id())
        );
        let comps = PrepComputations::new(xs[1].clone(), r, &xs);
        let reflectance = comps.schlick();
        assert!(eq_f64(reflectance, 0.04));
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_gt_n1() {
        let shape = ShapeContainer::from(Sphere::glassy());
        let r = Ray::new(Tuple::point(0.0, 0.99, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = intersections!(ShapeIntersection::new(1.8589, shape.clone(), shape.id()));
        let comps = PrepComputations::new(xs[0].clone(), r, &xs);
        let reflectance = comps.schlick();
        assert!(eq_f64(reflectance, 0.48873));
    }
}
