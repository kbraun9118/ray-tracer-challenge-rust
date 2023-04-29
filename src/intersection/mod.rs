extern crate self as ray_tracer_challenge;
use std::{collections::BinaryHeap, ops::Index, rc::Rc};

use crate::util::eq_f64;

use self::shape::Shape;

pub mod ray;
pub mod shape;
pub mod precomputation;

#[derive(Debug, Clone)]
pub struct Intersection {
    t: f64,
    object: Rc<dyn Shape>,
}

impl Intersection {
    pub fn new(t: f64, object: Rc<dyn Shape>) -> Self {
        Self { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &Rc<dyn Shape> {
        &self.object
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        eq_f64(self.t(), other.t())
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

#[derive(Debug)]
pub struct IntersectionHeap {
    inner: BinaryHeap<Intersection>,
}

impl IntersectionHeap {
    pub fn new() -> Self {
        Self {
            inner: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, i: Intersection) {
        self.inner.push(i);
    }

    pub fn hit(&mut self) -> Option<Intersection> {
        while let Some(i) = self.inner.pop() {
            if i.t().is_sign_positive() {
                return Some(i)
            }
        } 
        None
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn iter(&self) -> std::collections::binary_heap::Iter<Intersection> {
        self.inner.iter()
    }
}

impl IntoIterator for IntersectionHeap {
    type Item = Intersection;

    type IntoIter = std::collections::binary_heap::IntoIter<Intersection>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl Index<usize> for IntersectionHeap {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        let mut intersections = self.inner.iter().collect::<Vec<_>>();
        intersections.sort();
        intersections[intersections.len() - 1 - index]
    }
}

#[macro_export]
macro_rules! intersections {
    ( $( $x:expr ),* ) => {
        {
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
    use super::*;

    use std::rc::Rc;

    use crate::{
        intersection::{shape::sphere::Sphere, Intersection},
        util::eq_f64,
    };

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Rc::new(Sphere::new());
        let i = Intersection::new(3.5, s.clone());

        assert!(eq_f64(3.5, i.t()));
        assert_eq!(i.object().as_ref(), s.as_ref())
    }

    #[test]
    fn aggregating_intersections() {
        let s = Rc::new(Sphere::new());
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s.clone());

        let xs = intersections![i1, i2];
        assert_eq!(xs[0].object().as_ref(), s.as_ref());
        assert_eq!(xs[0].t(), 1.0);
        assert_eq!(xs[1].object().as_ref(), s.as_ref());
        assert_eq!(xs[1].t(), 2.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Rc::new(Sphere::new());
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s.clone());

        let mut xs = intersections![i1.clone(), i2];

        let hit = xs.hit();

        assert!(hit.is_some());
        assert_eq!(i1, hit.unwrap());
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Rc::new(Sphere::new());
        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(1.0, s.clone());

        let mut xs = intersections![i1, i2.clone()];

        let hit = xs.hit();

        assert!(hit.is_some());
        assert_eq!(i2, hit.unwrap());
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Rc::new(Sphere::new());
        let i1 = Intersection::new(-2.0, s.clone());
        let i2 = Intersection::new(-1.0, s.clone());

        let mut xs = intersections![i1, i2];

        let hit = xs.hit();

        assert!(hit.is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Rc::new(Sphere::new());
        let i1 = Intersection::new(5.0, s.clone());
        let i2 = Intersection::new(7.0, s.clone());
        let i3 = Intersection::new(-3.0, s.clone());
        let i4 = Intersection::new(2.0, s);

        let mut xs = intersections![i1, i2, i3, i4.clone()];

        let hit = xs.hit();

        assert!(hit.is_some());

        assert_eq!(i4, hit.unwrap());
    }
}
