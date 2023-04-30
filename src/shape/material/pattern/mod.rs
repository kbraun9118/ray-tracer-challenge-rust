use crate::{color::Color, tuple::Tuple};
use std::fmt::Debug;

pub mod stripes;
pub mod solid;

pub trait Pattern: Debug {
    fn color_at(&self, point: Tuple) -> Color;
}
