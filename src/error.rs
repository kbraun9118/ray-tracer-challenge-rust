use std::{error::Error, fmt::Display};

pub type RayTraceResult<T> = Result<T, RayTraceError>;

#[derive(Debug)]
pub enum RayTraceError {
    IoError(std::io::Error),
}

impl Display for RayTraceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => writeln!(f, "IO Error occurred: {}", e)
        }
    }
}

impl Error for RayTraceError {}

impl From<std::io::Error> for RayTraceError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
