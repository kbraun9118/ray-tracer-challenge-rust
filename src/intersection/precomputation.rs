use crate::{
    intersection::{ray::Ray, Intersection},
    shape::Shape,
    tuple::Tuple,
    util::EPSILON,
};

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct PreComputations {
    t: f64,
    object: Rc<dyn Shape>,
    point: Tuple,
    over_point: Tuple,
    eye_v: Tuple,
    normal_v: Tuple,
    inside: bool,
}

impl PreComputations {
    pub fn new(intersection: Intersection, ray: Ray) -> Self {
        let point = ray.position(intersection.t());
        let mut normal_v = intersection.object().normal_at(point);
        let eye_v = -ray.direction();
        let mut inside = false;

        if normal_v * eye_v < 0.0 {
            inside = true;
            normal_v = -normal_v
        }

        Self {
            t: intersection.t(),
            object: intersection.object().clone(),
            point,
            over_point: point + normal_v * EPSILON,
            eye_v,
            normal_v,
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

    pub fn inside(&self) -> bool {
        self.inside
    }
}

#[cfg(test)]
mod tests {

    use crate::{shape::sphere::Sphere, transformation::Transformation};

    use super::*;

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Rc::new(Sphere::new());
        let i = Intersection::new(4.0, s.clone());

        let comps = PreComputations::new(i.clone(), r);

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

        let comps = PreComputations::new(i.clone(), r);

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
        let comps = PreComputations::new(i.clone(), r);

        assert!(comps.over_point().z() < -EPSILON / 2.0);
        assert!(comps.point().z() > comps.over_point().z());
    }
}
