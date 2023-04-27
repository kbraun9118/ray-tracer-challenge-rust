use uuid::Uuid;

use std::{fmt::Debug};

use crate::{transformation::Transformation, tuple::Tuple};

use self::material::Material;

use super::ray::Ray;

pub mod sphere;
pub mod material;

pub trait Shape: Debug {
    fn id(&self) -> Uuid;
    fn intersects(&self, ray: Ray) -> Vec<f64>;
    fn transformation(&self) -> Transformation;
    fn set_transformation(&mut self, transformation: Transformation);
    fn material(&self) -> Material;
    fn set_material(&mut self, material: Material);
    fn normal_at(&self, point: Tuple) -> Tuple;
}

impl PartialEq for &dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}