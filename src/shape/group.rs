use std::{
    ops::Deref,
    sync::{Arc, RwLock, Weak},
};

use uuid::Uuid;

use crate::{
    intersection::{ray::Ray, Intersection, ShapeIntersection},
    transformation::Transformation,
    tuple::Tuple,
};

use super::{material::Material, BoundedBox, Shape, ShapeContainer};

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Difference,
    Intersection,
    Group,
    Union,
}

impl Operation {
    fn intersection_allowed(&self, lhit: bool, inl: bool, inr: bool) -> bool {
        match self {
            Operation::Union => (lhit && !inr) || (!lhit && !inl),
            Operation::Intersection => (lhit && inr) || (!lhit && inl),
            Operation::Difference => (lhit && !inr) || (!lhit && inl),
            _ => panic!("cannot determine intersections for non CSG"),
        }
    }
}

#[derive(Debug)]
pub struct Group {
    id: uuid::Uuid,
    shapes: Vec<ShapeContainer>,
    transformation: Transformation,
    parent: Option<WeakGroupContainer>,
    bounding_box: BoundedBox,
    operation: Operation,
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
            operation: Operation::Group,
        }
    }

    pub fn csg(
        operation: Operation,
        left: ShapeContainer,
        right: ShapeContainer,
    ) -> GroupContainer {
        if operation == Operation::Group {
            panic!("Cannot create CSG as Group");
        }
        let id = Uuid::new_v4();
        let group = Self {
            id,
            shapes: vec![],
            transformation: Transformation::default(),
            parent: None,
            bounding_box: BoundedBox::empty(),
            operation: Operation::Group,
        };
        let g = GroupContainer::from(group);
        g.add_child(left);
        g.add_child(right);
        g.write().unwrap().operation = operation;
        g
    }

    pub fn children(&self) -> Vec<ShapeContainer> {
        self.shapes.clone()
    }

    pub fn left(&self) -> ShapeContainer {
        if let Operation::Group = self.operation {
            panic!("Cannot access left of non csg group")
        } else {
            self.shapes[0].clone()
        }
    }

    pub fn right(&self) -> ShapeContainer {
        if let Operation::Group = self.operation {
            panic!("Cannot access left of non csg group")
        } else {
            self.shapes[1].clone()
        }
    }

    pub fn filter_intersections(&self, xs: &Vec<Intersection>) -> Vec<Intersection> {
        let mut inl = false;
        let mut inr = false;

        let mut result = vec![];

        for intersection in xs.iter() {
            let lhit = self.left().includes(intersection.object());

            if self.operation.intersection_allowed(lhit, inl, inr) {
                result.push(intersection.clone());
            }

            if lhit {
                inl = !inl
            } else {
                inr = !inr
            }
        }

        result
    }
}

impl Shape for Group {
    fn id(&self) -> uuid::Uuid {
        self.id
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        if !self.bounding_box.intersects(ray) {
            return vec![];
        }
        let mut xs: Vec<_> = self
            .shapes
            .iter()
            .flat_map(|s| s.read().unwrap().intersects(ray))
            .collect();

        xs.sort();
        xs.reverse();

        if self.operation == Operation::Group {
            xs
        } else {
            self.filter_intersections(&xs)
        }
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
            .filter_map(|s| s.read().unwrap().material(id))
            .next()
    }

    fn set_material(&mut self, _material: Material) {
        panic!("Group cannot have material")
    }

    fn local_normal_at(
        &self,
        id: Uuid,
        point: Tuple,
        intersection: ShapeIntersection,
    ) -> Option<Tuple> {
        self.shapes
            .iter()
            .filter_map(|s| {
                s.read()
                    .unwrap()
                    .local_normal_at(id, point, intersection.clone())
            })
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
            bbox.add_box(child.read().unwrap().parent_space_bounds());
        }
        bbox
    }

    fn contains(&self, id: Uuid) -> bool {
        self.children()
            .iter()
            .any(|s| s.read().unwrap().contains(id))
    }
}

#[derive(Debug, Clone)]
pub struct GroupContainer(Arc<RwLock<Group>>);

impl GroupContainer {
    pub fn add_child(&self, shape: ShapeContainer) {
        let mut group = self.0.write().unwrap();
        if group.operation != Operation::Group {
            panic!("Cannot add children to CSG");
        }
        let weak_container = Arc::downgrade(self);
        shape
            .write()
            .unwrap()
            .set_parent(WeakGroupContainer(weak_container));

        group.shapes.push(shape);
        group.bounding_box = group.bounds()
    }
}

impl Default for GroupContainer {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(Group::new())))
    }
}

impl From<Group> for GroupContainer {
    fn from(value: Group) -> Self {
        GroupContainer(Arc::new(RwLock::new(value)))
    }
}

impl Into<ShapeContainer> for GroupContainer {
    fn into(self) -> ShapeContainer {
        ShapeContainer(self.0)
    }
}

impl Deref for GroupContainer {
    type Target = Arc<RwLock<Group>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct WeakGroupContainer(Weak<RwLock<Group>>);

impl From<GroupContainer> for WeakGroupContainer {
    fn from(value: GroupContainer) -> Self {
        WeakGroupContainer(Arc::downgrade(&value.0))
    }
}

impl Deref for WeakGroupContainer {
    type Target = Weak<RwLock<Group>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        intersection::ray::Ray,
        shape::{cube::Cube, sphere::Sphere},
        tuple::Tuple,
    };

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
        let s_id = s.read().unwrap().id();
        g.add_child(s.clone());
        assert!(!g.read().unwrap().shapes.is_empty());
        assert!(g
            .read()
            .unwrap()
            .shapes
            .iter()
            .any(|s| s.read().unwrap().id() == s_id));
        let s_parent_id = s
            .read()
            .unwrap()
            .parent()
            .unwrap()
            .upgrade()
            .unwrap()
            .read()
            .unwrap()
            .id;
        assert_eq!(s_parent_id, g.read().unwrap().id());
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

        let xs = g.read().unwrap().local_intersect(r);

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

        let xs = g.read().unwrap().intersects(r);

        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn csg_is_create_with_an_operation_and_two_shapes() {
        let s1 = Sphere::new();
        let s1_id = s1.id();
        let s2 = Cube::new();
        let s2_id = s2.id();
        let c = Group::csg(Operation::Union, s1.into(), s2.into());
        let c_id = c.read().unwrap().id();

        assert_eq!(c.read().unwrap().operation, Operation::Union);
        assert_eq!(c.read().unwrap().left().id(), s1_id);
        assert_eq!(c.read().unwrap().right().id(), s2_id);
        assert_eq!(
            c.read()
                .unwrap()
                .left()
                .read()
                .unwrap()
                .parent()
                .unwrap()
                .upgrade()
                .unwrap()
                .read()
                .unwrap()
                .id(),
            c_id
        );
        assert_eq!(
            c.read()
                .unwrap()
                .right()
                .read()
                .unwrap()
                .parent()
                .unwrap()
                .upgrade()
                .unwrap()
                .read()
                .unwrap()
                .id(),
            c_id
        );
    }

    #[test]
    fn evaluating_the_rule_for_a_csg_operation() {
        let exs = vec![
            (Operation::Union, true, true, true, false),
            (Operation::Union, true, true, false, true),
            (Operation::Union, true, false, true, false),
            (Operation::Union, true, false, false, true),
            (Operation::Union, false, true, true, false),
            (Operation::Union, false, true, false, false),
            (Operation::Union, false, false, true, true),
            (Operation::Union, false, false, false, true),
            (Operation::Intersection, true, true, true, true),
            (Operation::Intersection, true, true, false, false),
            (Operation::Intersection, true, false, true, true),
            (Operation::Intersection, true, false, false, false),
            (Operation::Intersection, false, true, true, true),
            (Operation::Intersection, false, true, false, true),
            (Operation::Intersection, false, false, true, false),
            (Operation::Intersection, false, false, false, false),
            (Operation::Difference, true, true, true, false),
            (Operation::Difference, true, true, false, true),
            (Operation::Difference, true, false, true, false),
            (Operation::Difference, true, false, false, true),
            (Operation::Difference, false, true, true, true),
            (Operation::Difference, false, true, false, true),
            (Operation::Difference, false, false, true, false),
            (Operation::Difference, false, false, false, false),
        ];

        for (op, lhit, inl, inr, expected) in exs {
            let result = op.intersection_allowed(lhit, inl, inr);
            assert_eq!(expected, result)
        }
    }

    #[test]
    fn filtering_a_list_of_intersections() {
        let s1 = ShapeContainer::from(Sphere::new());
        let s1_id = s1.read().unwrap().id();
        let s2 = ShapeContainer::from(Cube::new());
        let s2_id = s2.read().unwrap().id();
        let xs = vec![
            Intersection::new(1.0, s1_id),
            Intersection::new(2.0, s2_id),
            Intersection::new(3.0, s1_id),
            Intersection::new(4.0, s2_id),
        ];
        let exs = vec![
            (Operation::Union, 0, 3),
            (Operation::Intersection, 1, 2),
            (Operation::Difference, 0, 1),
        ];

        for (op, x0, x1) in exs {
            let c = Group::csg(op, s1.clone(), s2.clone());

            let result = c.read().unwrap().filter_intersections(&xs);
            assert_eq!(result.len(), 2);
            assert_eq!(result[0], xs[x0]);
            assert_eq!(result[1], xs[x1]);
        }
    }

    #[test]
    fn a_ray_misses_a_csg_object() {
        let c = Group::csg(Operation::Union, Sphere::new().into(), Cube::new().into());
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = c.read().unwrap().local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_hits_a_csg_object() {
        let s1 = Sphere::new();
        let s1_id = s1.id();
        let mut s2 = Sphere::new();
        let s2_id = s2.id();
        s2.set_transformation(Transformation::identity().translation(0.0, 0.0, 0.5));
        let c = Group::csg(Operation::Union, s1.into(), s2.into());
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = c.read().unwrap().local_intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 4.0);
        assert_eq!(xs[0].object(), s1_id);
        assert_eq!(xs[1].t(), 6.5);
        assert_eq!(xs[1].object(), s2_id);
    }
}
