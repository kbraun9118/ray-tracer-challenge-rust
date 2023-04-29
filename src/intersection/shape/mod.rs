use uuid::Uuid;

use std::fmt::Debug;

use crate::{transformation::Transformation, tuple::Tuple};

use self::material::Material;

use super::ray::Ray;

pub mod material;
pub mod sphere;
pub mod plane;

pub trait Shape: Debug {
    fn id(&self) -> Uuid;
    fn local_intersect(&self, ray: Ray) -> Vec<f64>;
    fn transformation(&self) -> Transformation;
    fn set_transformation(&mut self, transformation: Transformation);
    fn material(&self) -> Material;
    fn set_material(&mut self, material: Material);
    fn local_normal_at(&self, point: Tuple) -> Tuple;

    fn intersects(&self, ray: Ray) -> Vec<f64> {
        let ray = self.transformation().inverse().unwrap() * ray;
        self.local_intersect(ray)
    }

    fn normal_at(&self, point: Tuple) -> Tuple {
        let object_point = self.transformation().inverse().unwrap() * point;
        let object_normal = self.local_normal_at(object_point);
        let mut world_normal = self.transformation().inverse().unwrap().transpose() * object_normal;
        world_normal.as_vector();
        world_normal.normalize()
    }
}

impl PartialEq for &dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestShape {
        id: Uuid,
        transformation: Transformation,
        material: Material,
    }

    impl TestShape {
        fn new() -> Self {
            Self {
                id: Uuid::new_v4(),
                transformation: Transformation::identity(),
                material: Material::new(),
            }
        }
    }

    impl Shape for TestShape {
        fn id(&self) -> Uuid {
            self.id
        }

        fn local_intersect(&self, ray: Ray) -> Vec<f64> {
            vec![ray.origin().x(), ray.origin().y(), ray.origin().z()]
        }

        fn transformation(&self) -> Transformation {
            self.transformation.clone()
        }

        fn set_transformation(&mut self, transformation: Transformation) {
            self.transformation = transformation;
        }

        fn material(&self) -> Material {
            self.material
        }

        fn set_material(&mut self, material: Material) {
            self.material = material;
        }

        fn local_normal_at(&self, point: Tuple) -> Tuple {
            let mut point = point;
            point.as_vector();
            point
        }
    }

    #[test]
    fn the_default_transformation() {
        let shape = TestShape::new();

        assert_eq!(shape.transformation(), Transformation::identity());
    }

    #[test]
    fn assigning_a_transformation() {
        let mut shape = TestShape::new();
        let transformation = Transformation::identity().translation(2.0, 3.0, 4.0);
        shape.set_transformation(transformation.clone());

        assert_eq!(shape.transformation(), transformation);
    }

    #[test]
    fn the_default_material() {
        let shape = TestShape::new();

        assert_eq!(shape.material(), Material::new());
    }

    #[test]
    fn assigning_a_material() {
        let mut shape = TestShape::new();
        let material = Material::new().with_ambient(1.0);
        shape.set_material(material.clone());

        assert_eq!(shape.material(), material);
    }

    #[test]
    fn intersects_scaled_shape_with_ray() {
        let ray = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let mut shape = TestShape::new();
        shape.set_transformation(Transformation::identity().scale(2.0, 2.0, 2.0).clone());
        let xs = shape.intersects(ray);
        
        assert_eq!(xs.len(), 3);
        assert_eq!(xs[0], 0.5);
        assert_eq!(xs[1], 1.0);
        assert_eq!(xs[2], 1.5);
    }

    #[test]
    fn computing_normal_on_translated_shape() {
        let mut shape = TestShape::new();
        shape.set_transformation(Transformation::identity().translation(0.0, 1.0, 0.0).clone());
        let normal = shape.normal_at(Tuple::point(0.0, 1.70711, -0.70711));

        assert_eq!(normal, Tuple::vector(0.0, 0.70711, -0.70711));
    }
}
