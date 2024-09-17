use bounded_box::BoundedBox;
use group::WeakGroupContainer;
use uuid::Uuid;

use std::{cell::RefCell, fmt::Debug, ops::Deref, rc::Rc};

use crate::{intersection::Intersection, transformation::Transformation, tuple::Tuple};

use self::material::Material;

use crate::intersection::ray::Ray;

pub mod bounded_box;
pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod group;
pub mod material;
pub mod plane;
pub mod sphere;
pub mod triangle;

#[derive(Debug, Clone)]
pub struct ShapeContainer(Rc<RefCell<dyn Shape>>);

impl ShapeContainer {
    pub fn id(&self) -> Uuid {
        self.borrow().id()
    }
}

impl<T: Shape + 'static> From<T> for ShapeContainer {
    fn from(value: T) -> Self {
        ShapeContainer(Rc::new(RefCell::new(value)))
    }
}

impl Deref for ShapeContainer {
    type Target = Rc<RefCell<dyn Shape>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for ShapeContainer {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

pub trait Shape: Debug {
    fn id(&self) -> Uuid;
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection>;
    fn transformation(&self) -> Transformation;
    fn set_transformation(&mut self, transformation: Transformation);
    fn material(&self, id: Uuid) -> Option<Material>;
    fn set_material(&mut self, material: Material);
    fn local_normal_at(&self, id: uuid::Uuid, point: Tuple) -> Option<Tuple>;
    fn parent(&self) -> Option<WeakGroupContainer>;
    fn set_parent(&mut self, parent: WeakGroupContainer);
    fn bounds(&self) -> BoundedBox;

    fn intersects(&self, ray: Ray) -> Vec<Intersection> {
        let ray = self.transformation().inverse().unwrap() * ray;
        self.local_intersect(ray)
    }

    fn normal_at(&self, id: uuid::Uuid, point: Tuple) -> Option<Tuple> {
        let local_point = self.world_to_object(point);
        self.local_normal_at(id, local_point)
            .map(|local_normal| self.normal_to_world(local_normal))
    }

    fn world_to_object(&self, point: Tuple) -> Tuple {
        let mut point = point;
        if let Some(parent) = self.parent() {
            point = parent.upgrade().unwrap().borrow().world_to_object(point);
        }

        self.transformation()
            .inverse()
            .expect("Could not get inverse")
            * point
    }

    fn normal_to_world(&self, normal: Tuple) -> Tuple {
        let mut normal = self
            .transformation()
            .inverse()
            .expect("Could not find inverse")
            .transpose()
            * normal;
        normal.as_vector();
        let mut normal = normal.normalize();

        if let Some(parent) = self.parent() {
            let parent = parent.upgrade().unwrap();
            normal = parent.borrow().normal_to_world(normal);
        }

        normal
    }

    fn parent_space_bounds(&self) -> BoundedBox {
        self.bounds().transform(self.transformation())
    }
}

impl PartialEq for &dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

#[cfg(test)]
mod tests {
    use core::f64;

    use group::{Group, GroupContainer};
    use sphere::Sphere;

    use super::*;

    #[derive(Debug)]
    struct TestShape {
        id: Uuid,
        transformation: Transformation,
        material: Material,
        parent: Option<WeakGroupContainer>,
    }

    impl TestShape {
        fn new() -> Self {
            Self {
                id: Uuid::new_v4(),
                transformation: Transformation::identity(),
                material: Material::new(),
                parent: None,
            }
        }
    }

    impl Shape for TestShape {
        fn id(&self) -> Uuid {
            self.id
        }

        fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
            vec![
                Intersection::new(ray.origin().x(), self.id),
                Intersection::new(ray.origin().y(), self.id),
                Intersection::new(ray.origin().z(), self.id),
            ]
        }

        fn transformation(&self) -> Transformation {
            self.transformation.clone()
        }

        fn set_transformation(&mut self, transformation: Transformation) {
            self.transformation = transformation;
        }

        fn material(&self, id: Uuid) -> Option<Material> {
            if id == self.id {
                Some(self.material.clone())
            } else {
                None
            }
        }

        fn set_material(&mut self, material: Material) {
            self.material = material;
        }

        fn local_normal_at(&self, id: Uuid, point: Tuple) -> Option<Tuple> {
            if id != self.id {
                None
            } else {
                let mut point = point;
                point.as_vector();
                Some(point)
            }
        }

        fn parent(&self) -> Option<WeakGroupContainer> {
            self.parent.clone()
        }

        fn set_parent(&mut self, parent: WeakGroupContainer) {
            self.parent = Some(parent.clone())
        }

        fn bounds(&self) -> BoundedBox {
            BoundedBox::new(Tuple::origin(), Tuple::origin())
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

        assert_eq!(shape.material(shape.id()).unwrap(), Material::new());
    }

    #[test]
    fn assigning_a_material() {
        let mut shape = TestShape::new();
        let material = Material::new().with_ambient(1.0);
        shape.set_material(material.clone());

        assert_eq!(shape.material(shape.id()).unwrap(), material);
    }

    #[test]
    fn intersects_scaled_shape_with_ray() {
        let ray = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let mut shape = TestShape::new();
        shape.set_transformation(Transformation::identity().scale(2.0, 2.0, 2.0).clone());
        let xs = shape.intersects(ray);

        assert_eq!(xs.len(), 3);
        assert_eq!(xs[0].t(), 0.5);
        assert_eq!(xs[1].t(), 1.0);
        assert_eq!(xs[2].t(), 1.5);
    }

    #[test]
    fn computing_normal_on_translated_shape() {
        let mut shape = TestShape::new();
        shape.set_transformation(
            Transformation::identity()
                .translation(0.0, 1.0, 0.0)
                .clone(),
        );
        let normal = shape
            .normal_at(shape.id(), Tuple::point(0.0, 1.70711, -0.70711))
            .unwrap();

        assert_eq!(normal, Tuple::vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn a_shape_has_a_parent_attribute() {
        let s = TestShape::new();

        assert!(s.parent().is_none())
    }

    #[test]
    fn converting_a_point_from_world_to_object_space() {
        let mut g1 = Group::new();
        g1.set_transformation(Transformation::identity().rotate_y(f64::consts::PI / 2.0));
        let mut g2 = Group::new();
        g2.set_transformation(Transformation::identity().scale(2.0, 2.0, 2.0));
        let mut s = Sphere::new();
        s.set_transformation(Transformation::identity().translation(5.0, 0.0, 0.0));
        let s = ShapeContainer::from(s);
        let g2 = GroupContainer::from(g2);
        g2.add_child(s.clone());
        let g1 = GroupContainer::from(g1);
        g1.add_child(g2.into());

        let p = s.borrow().world_to_object(Tuple::point(-2.0, 0.0, -10.0));

        assert_eq!(p, Tuple::point(0.0, 0.0, -1.0));
    }

    #[test]
    fn converting_a_normal_from_object_to_world_space() {
        let mut g1 = Group::new();
        g1.set_transformation(Transformation::identity().rotate_y(f64::consts::PI / 2.0));
        let mut g2 = Group::new();
        g2.set_transformation(Transformation::identity().scale(1.0, 2.0, 3.0));
        let g1 = GroupContainer::from(g1);
        let mut s = Sphere::new();
        let g2 = GroupContainer::from(g2);
        s.set_transformation(Transformation::identity().translation(5.0, 0.0, 0.0));
        let s = ShapeContainer::from(s);
        g2.add_child(s.clone());
        g1.add_child(g2.into());

        let n = s.borrow().normal_to_world(Tuple::vector(
            3f64.sqrt() / 3.0,
            3f64.sqrt() / 3.0,
            3f64.sqrt() / 3.0,
        ));

        assert_eq!(n, Tuple::vector(0.28571, 0.42857, -0.85714));
    }

    #[test]
    fn finding_the_normal_on_a_child_object() {
        let mut g1 = Group::new();
        g1.set_transformation(Transformation::identity().rotate_y(f64::consts::PI / 2.0));
        let mut g2 = Group::new();
        g2.set_transformation(Transformation::identity().scale(1.0, 2.0, 3.0));
        let g1 = GroupContainer::from(g1);
        let mut s = Sphere::new();
        s.set_transformation(Transformation::identity().translation(5.0, 0.0, 0.0));
        let s = ShapeContainer::from(s);
        let g2 = GroupContainer::from(g2);
        g2.add_child(s.clone());
        g1.add_child(g2.into());

        let n = s
            .borrow()
            .normal_at(s.id(), Tuple::point(1.7321, 1.1547, -5.5774))
            .unwrap();

        assert_eq!(n, Tuple::vector(0.28570, 0.42854, -0.85716));
    }
}
