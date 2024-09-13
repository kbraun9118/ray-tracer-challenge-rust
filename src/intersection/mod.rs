extern crate self as ray_tracer_challenge;

use std::{collections::BinaryHeap, ops::Index};

use uuid::Uuid;

use crate::{shape::ShapeContainer, util::eq_f64};

pub mod prepcomputation;
pub mod ray;

#[derive(Debug, Clone)]
pub struct Intersection {
    t: f64,
    object: uuid::Uuid,
}

impl Intersection {
    pub fn new(t: f64, object: uuid::Uuid) -> Self {
        Self { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> uuid::Uuid {
        self.object.clone()
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.object == other.object && eq_f64(self.t(), other.t())
    }
}

impl Eq for Intersection {}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        if eq_f64(self.t(), other.t()) {
            Some(Equal)
        } else {
            Some(if self.t() < other.t() { Greater } else { Less })
        }
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct ShapeIntersection {
    t: f64,
    object: ShapeContainer,
    object_id: Uuid,
}

impl ShapeIntersection {
    pub fn new(t: f64, object: ShapeContainer, object_id: Uuid) -> Self {
        Self {
            t,
            object,
            object_id,
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
}

impl PartialEq for ShapeIntersection {
    fn eq(&self, other: &Self) -> bool {
        self.object.id() == other.object.id() && eq_f64(self.t(), other.t())
    }
}

impl Eq for ShapeIntersection {}

impl PartialOrd for ShapeIntersection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        if eq_f64(self.t(), other.t()) {
            Some(Equal)
        } else {
            Some(if self.t() < other.t() { Greater } else { Less })
        }
    }
}

impl Ord for ShapeIntersection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
pub struct IntersectionHeap {
    inner: BinaryHeap<ShapeIntersection>,
}

impl IntersectionHeap {
    pub fn new() -> Self {
        Self {
            inner: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, i: ShapeIntersection) {
        self.inner.push(i);
    }

    pub fn hit(&self) -> Option<ShapeIntersection> {
        for i in 0..self.len() {
            let i = &self[i];
            if i.t.is_sign_positive() {
                return Some(i.clone());
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn iter(&self) -> std::collections::binary_heap::Iter<ShapeIntersection> {
        self.inner.iter()
    }
}

impl IntoIterator for IntersectionHeap {
    type Item = ShapeIntersection;

    type IntoIter = std::collections::binary_heap::IntoIter<ShapeIntersection>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl Index<usize> for IntersectionHeap {
    type Output = ShapeIntersection;

    fn index(&self, index: usize) -> &Self::Output {
        let mut intersections = self.inner.iter().collect::<Vec<_>>();
        intersections.sort();
        intersections[intersections.len() - 1 - index]
    }
}

impl FromIterator<ShapeIntersection> for IntersectionHeap {
    fn from_iter<T: IntoIterator<Item = ShapeIntersection>>(iter: T) -> Self {
        let mut heap = IntersectionHeap::new();
        for i in iter {
            heap.push(i);
        }
        heap
    }
}

#[macro_export]
macro_rules! intersections {
    ( $( $x:expr ),* ) => {
        {
            extern crate self as ray_tracer_challenge;
            use ray_tracer_challenge::intersection::IntersectionHeap;
            let mut temp_inter = IntersectionHeap::new();
            $(
                temp_inter.push($x);
            )*
            temp_inter
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{shape::sphere::Sphere, util::eq_f64};

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = ShapeContainer::from(Sphere::new());
        let i = ShapeIntersection::new(3.5, s.clone(), s.id());

        assert!(eq_f64(3.5, i.t()));
        assert_eq!(i.object(), s.clone());
    }

    #[test]
    fn aggregating_intersections() {
        let s = ShapeContainer::from(Sphere::new());
        let i1 = ShapeIntersection::new(1.0, s.clone(), s.id());
        let i2 = ShapeIntersection::new(2.0, s.clone(), s.id());

        let xs = intersections![i1, i2];
        assert_eq!(xs[0].object(), s.clone());
        assert_eq!(xs[0].t(), 1.0);
        assert_eq!(xs[1].object(), s.clone());
        assert_eq!(xs[1].t(), 2.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = ShapeContainer::from(Sphere::new());
        let i1 = ShapeIntersection::new(1.0, s.clone(), s.id());
        let i2 = ShapeIntersection::new(2.0, s.clone(), s.id());

        let xs = intersections![i1.clone(), i2];

        let hit = xs.hit();

        assert!(hit.is_some());
        assert_eq!(i1, hit.unwrap());
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = ShapeContainer::from(Sphere::new());
        let i1 = ShapeIntersection::new(-1.0, s.clone(), s.id());
        let i2 = ShapeIntersection::new(1.0, s.clone(), s.id());

        let xs = intersections![i1, i2.clone()];

        let hit = xs.hit();

        assert!(hit.is_some());
        assert_eq!(i2, hit.unwrap());
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = ShapeContainer::from(Sphere::new());
        let i1 = ShapeIntersection::new(-2.0, s.clone(), s.id());
        let i2 = ShapeIntersection::new(-1.0, s.clone(), s.id());

        let xs = intersections![i1, i2];

        let hit = xs.hit();

        assert!(hit.is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = ShapeContainer::from(Sphere::new());
        let i1 = ShapeIntersection::new(5.0, s.clone(), s.id());
        let i2 = ShapeIntersection::new(7.0, s.clone(), s.id());
        let i3 = ShapeIntersection::new(-3.0, s.clone(), s.id());
        let i4 = ShapeIntersection::new(2.0, s.clone(), s.id());

        let xs = intersections![i1, i2, i3, i4.clone()];

        let hit = xs.hit();

        assert!(hit.is_some());

        assert_eq!(i4, hit.unwrap());
    }
}
