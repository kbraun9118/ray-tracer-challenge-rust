use std::{error::Error, fmt::Display};

use crate::tuple::Tuple;

pub type RayTraceResult<T> = Result<T, RayTraceError>;

#[derive(Debug)]
pub enum RayTraceError {
    IoError(std::io::Error),
    RayCreationError(Tuple, Tuple),
}

impl Display for RayTraceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RayTraceError::*;

        match self {
            IoError(e) => writeln!(f, "IO Error occurred: {}", e),
            RayCreationError(origin, point) => writeln!(
                f,
                "Could not create ray. {origin:?} must be a point and {point:?} must be a vector"
            ),
        }
    }
}

impl Error for RayTraceError {}

impl From<std::io::Error> for RayTraceError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
