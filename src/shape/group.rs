use crate::transformation::Transformation;

use super::Shape;

#[derive(Debug)]
pub struct Group {
    id: uuid::Uuid,
    shapes: Vec<*const dyn Shape>,
    transformation: Transformation,
    parent: Option<*const dyn Shape>,
}

impl Group {
    pub fn new() -> Self {
        let id = uuid::Uuid::new_v4();
        Self {
            id,
            shapes: vec![],
            transformation: Transformation::default(),
            parent: None,
        }
    }

    pub fn add_shape(&mut self, shape: *const dyn Shape) {
        unsafe {
            (shape as *mut dyn Shape).as_mut().unwrap().set_parent(self);
        }
        self.shapes.push(shape);
    }
}

impl Shape for Group {
    fn id(&self) -> uuid::Uuid {
        self.id
    }

    fn local_intersect(&self, ray: crate::intersection::ray::Ray) -> Vec<f64> {
        todo!()
    }

    fn transformation(&self) -> crate::transformation::Transformation {
        self.transformation.clone()
    }

    fn set_transformation(&mut self, transformation: crate::transformation::Transformation) {
        self.transformation = transformation;
    }

    fn material(&self) -> super::material::Material {
        todo!()
    }

    fn set_material(&mut self, material: super::material::Material) {
        todo!()
    }

    fn local_normal_at(&self, point: crate::tuple::Tuple) -> crate::tuple::Tuple {
        todo!()
    }

    fn parent(&self) -> Option<*const dyn Shape> {
        self.parent
    }

    fn set_parent(&mut self, parent: *const dyn Shape) {
        self.parent = Some(parent);
    }
}

#[cfg(test)]
mod tests {
    use std::ptr::eq;

    use crate::shape::sphere::Sphere;

    use super::*;

    #[test]
    fn creating_a_new_group() {
        let g = Group::new();

        assert_eq!(g.transformation(), Transformation::identity());
        assert!(g.shapes.is_empty());
    }

    #[test]
    fn a_shape_has_a_parent_attribute() {
        let s = Sphere::new();
        assert!(s.parent().is_none())
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let mut g = Group::new();
        let s = Sphere::new();
        g.add_shape(&s);
        assert!(!g.shapes.is_empty());
        assert!(g.shapes.contains(&(&s as *const dyn Shape)));
        assert_eq!(s.parent().unwrap() as *const Group, &g as *const Group)
    }
}
