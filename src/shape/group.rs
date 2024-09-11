use crate::transformation::Transformation;

use super::{BoundedBox, Shape};

#[derive(Debug)]
pub struct Group {
    id: uuid::Uuid,
    shapes: Vec<*mut dyn Shape>,
    transformation: Transformation,
    parent: Option<*mut dyn Shape>,
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

    pub fn add_child(&mut self, shape: *mut dyn Shape) {
        unsafe {
            shape
                .as_mut()
                .expect("Could not add shape")
                .set_parent(self);
        }
        self.shapes.push(shape);
    }
}

impl Shape for Group {
    fn id(&self) -> uuid::Uuid {
        self.id
    }

    fn local_intersect(&self, ray: crate::intersection::ray::Ray) -> Vec<f64> {
        let mut xs = vec![];

        for shape in &self.shapes {
            let mut shape_xs = unsafe {
                shape
                    .as_ref()
                    .expect("Could not find local intersection")
                    .intersects(ray)
            };
            xs.append(&mut shape_xs);
        }
        xs
    }

    fn transformation(&self) -> crate::transformation::Transformation {
        self.transformation.clone()
    }

    fn set_transformation(&mut self, transformation: crate::transformation::Transformation) {
        self.transformation = transformation;
    }

    fn material(&self) -> super::material::Material {
        panic!("Group cannot have material")
    }

    fn set_material(&mut self, _material: super::material::Material) {
        panic!("Group cannot have material")
    }

    fn local_normal_at(&self, _point: crate::tuple::Tuple) -> crate::tuple::Tuple {
        panic!("Cannot find local normal of group")
    }

    fn parent(&self) -> Option<*mut dyn Shape> {
        self.parent
    }

    fn set_parent(&mut self, parent: *mut dyn Shape) {
        self.parent = Some(parent);
    }

    fn bounds(&self) -> BoundedBox {
        panic!("Will look at this later")
    }
}

#[cfg(test)]
mod tests {
    use crate::{intersection::ray::Ray, shape::sphere::Sphere, tuple::Tuple};

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
        let mut s = Sphere::new();
        g.add_child(&mut s);
        assert!(!g.shapes.is_empty());
        assert!(g.shapes.contains(&(&mut s as *mut dyn Shape)));
        unsafe {
            assert_eq!(s.parent().unwrap().as_ref().unwrap(), &g);
        }
    }

    #[test]
    fn intersecting_a_ray_with_an_emtpy_group() {
        let g = Group::new();
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

        let xs = g.local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn intersecting_a_ray_with_a_nonempty_group() {
        let mut g = Group::new();
        let mut s1 = Sphere::new();
        let mut s2 = Sphere::new();
        s2.set_transformation(Transformation::default().translation(0.0, 0.0, -3.0));
        let mut s3 = Sphere::new();
        s3.set_transformation(Transformation::default().translation(5.0, 0.0, 0.0));
        g.add_child(&mut s1);
        g.add_child(&mut s2);
        g.add_child(&mut s3);
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let xs = g.local_intersect(r);

        assert_eq!(xs.len(), 4);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let mut g = Group::new();
        g.set_transformation(Transformation::identity().scale(2.0, 2.0, 2.0));
        let mut s = Sphere::new();
        s.set_transformation(Transformation::identity().translation(5.0, 0.0, 0.0));
        g.add_child(&mut s);
        let r = Ray::new(Tuple::point(10.0, 0.0, -10.0), Tuple::vector(0.0, 0.0, 1.0));

        let xs = g.intersects(r);

        assert_eq!(xs.len(), 2);
    }
}
