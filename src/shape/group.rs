use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

use uuid::Uuid;

use crate::{
    intersection::{ray::Ray, Intersection},
    transformation::Transformation,
    tuple::Tuple,
};

use super::{material::Material, BoundedBox, Shape, ShapeContainer};

#[derive(Debug)]
pub struct Group {
    id: uuid::Uuid,
    shapes: Vec<ShapeContainer>,
    transformation: Transformation,
    parent: Option<WeakGroupContainer>,
    bounding_box: BoundedBox,
}

impl Group {
    pub fn new() -> Self {
        let id = uuid::Uuid::new_v4();
        Self {
            id,
            shapes: vec![],
            transformation: Transformation::default(),
            parent: None,
            bounding_box: BoundedBox::empty(),
        }
    }
}

impl Shape for Group {
    fn id(&self) -> uuid::Uuid {
        self.id
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut xs = vec![];
        if self.bounding_box.intersects(ray) {
            for shape in &self.shapes {
                let mut shape_xs = shape.borrow().intersects(ray);
                xs.append(&mut shape_xs);
            }
        }
        xs
    }

    fn transformation(&self) -> Transformation {
        self.transformation.clone()
    }

    fn set_transformation(&mut self, transformation: Transformation) {
        self.transformation = transformation;
    }

    fn material(&self, id: Uuid) -> Option<Material> {
        self.shapes
            .iter()
            .filter_map(|s| s.borrow().material(id))
            .next()
    }

    fn set_material(&mut self, _material: Material) {
        panic!("Group cannot have material")
    }

    fn local_normal_at(&self, id: Uuid, point: Tuple) -> Option<Tuple> {
        self.shapes
            .iter()
            .filter_map(|s| s.borrow().local_normal_at(id, point))
            .next()
    }

    fn parent(&self) -> Option<WeakGroupContainer> {
        self.parent.clone()
    }

    fn set_parent(&mut self, parent: WeakGroupContainer) {
        self.parent = Some(parent.clone());
    }

    fn bounds(&self) -> BoundedBox {
        let mut bbox = BoundedBox::empty();
        for child in &self.shapes {
            bbox.add_box(child.borrow().parent_space_bounds());
        }
        bbox
    }
}

#[derive(Debug, Clone)]
pub struct GroupContainer(Rc<RefCell<Group>>);

impl GroupContainer {
    pub fn add_child(&self, shape: ShapeContainer) {
        let weak_container = Rc::downgrade(self);
        shape
            .borrow_mut()
            .set_parent(WeakGroupContainer(weak_container));

        let mut group = self.borrow_mut();
        group.shapes.push(shape);
        group.bounding_box = group.bounds()
    }
}

impl Default for GroupContainer {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Group::new())))
    }
}

impl From<Group> for GroupContainer {
    fn from(value: Group) -> Self {
        GroupContainer(Rc::new(RefCell::new(value)))
    }
}

impl Into<ShapeContainer> for GroupContainer {
    fn into(self) -> ShapeContainer {
        ShapeContainer(self.0)
    }
}

impl Deref for GroupContainer {
    type Target = Rc<RefCell<Group>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct WeakGroupContainer(Weak<RefCell<Group>>);

impl From<GroupContainer> for WeakGroupContainer {
    fn from(value: GroupContainer) -> Self {
        WeakGroupContainer(Rc::downgrade(&value.0))
    }
}

impl Deref for WeakGroupContainer {
    type Target = Weak<RefCell<Group>>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
        let g = GroupContainer::from(Group::new());
        let s = ShapeContainer::from(Sphere::new());
        let s_id = s.borrow().id();
        g.add_child(s.clone());
        assert!(!g.borrow().shapes.is_empty());
        assert!(g.borrow().shapes.iter().any(|s| s.borrow().id() == s_id));
        let s_parent_id = s.borrow().parent().unwrap().upgrade().unwrap().borrow().id;
        assert_eq!(s_parent_id, g.borrow().id());
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
        let g = Group::new();
        let s1 = Sphere::new();
        let mut s2 = Sphere::new();
        s2.set_transformation(Transformation::default().translation(0.0, 0.0, -3.0));
        let mut s3 = Sphere::new();
        s3.set_transformation(Transformation::default().translation(5.0, 0.0, 0.0));
        let g = GroupContainer::from(g);
        g.add_child(s1.into());
        g.add_child(s2.into());
        g.add_child(s3.into());
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let xs = g.borrow().local_intersect(r);

        assert_eq!(xs.len(), 4);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let mut g = Group::new();
        g.set_transformation(Transformation::identity().scale(2.0, 2.0, 2.0));
        let mut s = Sphere::new();
        s.set_transformation(Transformation::identity().translation(5.0, 0.0, 0.0));
        let g = GroupContainer::from(g);
        g.add_child(s.into());
        let r = Ray::new(Tuple::point(10.0, 0.0, -10.0), Tuple::vector(0.0, 0.0, 1.0));

        let xs = g.borrow().intersects(r);

        assert_eq!(xs.len(), 2);
    }
}
