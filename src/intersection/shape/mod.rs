use uuid::Uuid;

use std::fmt::Debug;

use crate::{transformation::Transformation, tuple::Tuple};

use super::ray::Ray;

pub mod sphere;

pub trait Shape: Debug {
    fn id(&self) -> Uuid;
    fn intersects(&self, ray: Ray) -> Vec<f64>;
    fn transformation(&self) -> Transformation;
    fn with_transformation(&mut self, transformation: Transformation);
    fn normal_at(&self, point: Tuple) -> Tuple;
}
