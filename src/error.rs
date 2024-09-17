use std::{error::Error, fmt::Display};

use crate::tuple::Tuple;

pub type RayTraceResult<T> = Result<T, RayTraceError>;

#[derive(Debug)]
pub enum RayTraceError {
    IoError(std::io::Error),
    RayCreationError(Tuple, Tuple),
    ParseFloatError(std::num::ParseFloatError),
    ParseIntError(std::num::ParseIntError),
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
            ParseFloatError(e) => writeln!(f, "ParseFloatError occured: {}", e),
            ParseIntError(e) => writeln!(f, "ParseIntError occured: {}", e),
        }
    }
}

impl Error for RayTraceError {}

impl From<std::io::Error> for RayTraceError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<std::num::ParseFloatError> for RayTraceError {
    fn from(value: std::num::ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

impl From<std::num::ParseIntError> for RayTraceError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}
