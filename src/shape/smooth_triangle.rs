use uuid::Uuid;

use crate::{
    intersection::{ray::Ray, Intersection, ShapeIntersection},
    transformation::Transformation,
    tuple::Tuple,
};

use super::{
    bounded_box::BoundedBox, group::WeakGroupContainer, material::Material, triangle::Triangle,
    Shape,
};

#[derive(Debug)]
pub struct SmoothTriangle {
    triangle: Triangle,
    n1: Tuple,
    n2: Tuple,
    n3: Tuple,
}

impl SmoothTriangle {
    pub fn new(p1: Tuple, p2: Tuple, p3: Tuple, n1: Tuple, n2: Tuple, n3: Tuple) -> Self {
        Self {
            triangle: Triangle::new(p1, p2, p3),
            n1,
            n2,
            n3,
        }
    }
}

impl Shape for SmoothTriangle {
    fn id(&self) -> Uuid {
        self.triangle.id()
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        self.triangle
            .local_intersect_with_uv(ray)
            .map(|(i, u, v)| vec![Intersection::new_with_uv(i.t(), i.object(), u, v)])
            .unwrap_or_default()
    }

    fn transformation(&self) -> Transformation {
        self.triangle.transformation()
    }

    fn set_transformation(&mut self, transformation: Transformation) {
        self.triangle.set_transformation(transformation);
    }

    fn material(&self, id: Uuid) -> Option<Material> {
        self.triangle.material(id)
    }

    fn set_material(&mut self, material: Material) {
        self.triangle.set_material(material);
    }

    fn local_normal_at(
        &self,
        id: Uuid,
        _point: Tuple,
        intersection: ShapeIntersection,
    ) -> Option<Tuple> {
        if id == self.id() {
            if let (Some(u), Some(v)) = (intersection.u(), intersection.v()) {
                Some(self.n2 * u + self.n3 * v + self.n1 * (1.0 - u - v))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parent(&self) -> Option<WeakGroupContainer> {
        self.triangle.parent()
    }

    fn set_parent(&mut self, parent: WeakGroupContainer) {
        self.triangle.set_parent(parent);
    }

    fn bounds(&self) -> BoundedBox {
        self.triangle.bounds()
    }

    fn contains(&self, id: Uuid) -> bool {
        self.triangle.id() == id
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        intersection::{prepcomputation::PrepComputations, ray::Ray, ShapeIntersection},
        intersections,
        shape::ShapeContainer,
        util::eq_f64,
    };

    use super::*;

    #[test]
    fn constructing_a_smooth_triangle() {
        let p1: Tuple = Tuple::point(0.0, 1.0, 0.0);
        let p2: Tuple = Tuple::point(-1.0, 0.0, 0.0);
        let p3: Tuple = Tuple::point(1.0, 0.0, 0.0);
        let n1: Tuple = Tuple::vector(0.0, 1.0, 0.0);
        let n2: Tuple = Tuple::vector(-1.0, 0.0, 0.0);
        let n3: Tuple = Tuple::vector(1.0, 0.0, 0.0);
        let t: SmoothTriangle = SmoothTriangle::new(p1, p2, p3, n1, n2, n3);

        assert_eq!(t.triangle.p1(), p1);
        assert_eq!(t.triangle.p2(), p2);
        assert_eq!(t.triangle.p3(), p3);
        assert_eq!(t.n1, n1);
        assert_eq!(t.n2, n2);
        assert_eq!(t.n3, n3);
    }

    #[test]
    fn an_intersection_with_a_smooth_triangle_stores_uv() {
        let p1: Tuple = Tuple::point(0.0, 1.0, 0.0);
        let p2: Tuple = Tuple::point(-1.0, 0.0, 0.0);
        let p3: Tuple = Tuple::point(1.0, 0.0, 0.0);
        let n1: Tuple = Tuple::vector(0.0, 1.0, 0.0);
        let n2: Tuple = Tuple::vector(-1.0, 0.0, 0.0);
        let n3: Tuple = Tuple::vector(1.0, 0.0, 0.0);
        let t: SmoothTriangle = SmoothTriangle::new(p1, p2, p3, n1, n2, n3);

        let r = Ray::new(Tuple::point(-0.2, 0.3, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = t.local_intersect(r);
        assert!(eq_f64(xs[0].u().unwrap(), 0.45));
        assert!(eq_f64(xs[0].v().unwrap(), 0.25));
    }

    #[test]
    fn a_smooth_triangle_uses_uv_to_interpolate_the_normal() {
        let p1: Tuple = Tuple::point(0.0, 1.0, 0.0);
        let p2: Tuple = Tuple::point(-1.0, 0.0, 0.0);
        let p3: Tuple = Tuple::point(1.0, 0.0, 0.0);
        let n1: Tuple = Tuple::vector(0.0, 1.0, 0.0);
        let n2: Tuple = Tuple::vector(-1.0, 0.0, 0.0);
        let n3: Tuple = Tuple::vector(1.0, 0.0, 0.0);
        let t = ShapeContainer::from(SmoothTriangle::new(p1, p2, p3, n1, n2, n3));

        let i = ShapeIntersection::new_with_uv(
            1.0,
            t.clone(),
            t.read().unwrap().id(),
            Some(0.45),
            Some(0.25),
        );

        let n = t
            .read()
            .unwrap()
            .normal_at(t.id(), Tuple::point(0.0, 0.0, 0.0), i);

        assert_eq!(n.unwrap(), Tuple::vector(-0.5547, 0.83205, 0.0));
    }

    #[test]
    fn preparing_the_normal_on_a_smooth_triangle() {
        let p1: Tuple = Tuple::point(0.0, 1.0, 0.0);
        let p2: Tuple = Tuple::point(-1.0, 0.0, 0.0);
        let p3: Tuple = Tuple::point(1.0, 0.0, 0.0);
        let n1: Tuple = Tuple::vector(0.0, 1.0, 0.0);
        let n2: Tuple = Tuple::vector(-1.0, 0.0, 0.0);
        let n3: Tuple = Tuple::vector(1.0, 0.0, 0.0);
        let t = ShapeContainer::from(SmoothTriangle::new(p1, p2, p3, n1, n2, n3));

        let i = ShapeIntersection::new_with_uv(
            1.0,
            t.clone(),
            t.read().unwrap().id(),
            Some(0.45),
            Some(0.25),
        );
        let r = Ray::new(Tuple::point(-0.2, 0.3, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = intersections![i.clone()];

        let comps = PrepComputations::new(i, r, &xs);

        assert_eq!(comps.normal_v(), Tuple::vector(-0.5547, 0.83205, 0.0));
    }
}
