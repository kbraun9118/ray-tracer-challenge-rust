use crate::{
    error::{RayTraceError, RayTraceResult},
    tuple::Tuple,
};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    pub fn try_new(origin: Tuple, direction: Tuple) -> RayTraceResult<Self> {
        if origin.is_point() && direction.is_vector() {
            Ok(Self { origin, direction })
        } else {
            Err(RayTraceError::RayCreationError(origin, direction))
        }
    }

    pub fn origin(&self) -> Tuple {
        self.origin
    }

    pub fn direciton(&self) -> Tuple {
        self.direction
    }

    pub fn position(&self, position: f64) -> Tuple {
        self.origin + (self.direction * position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);
        let r = Ray::try_new(origin, direction);

        assert!(r.is_ok());

        let r = r.unwrap();

        assert_eq!(origin, r.origin());
        assert_eq!(direction, r.direciton());
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::try_new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0)).unwrap();
        assert_eq!(Tuple::point(2.0, 3.0, 4.0), r.position(0.0));
        assert_eq!(Tuple::point(3.0, 3.0, 4.0), r.position(1.0));
        assert_eq!(Tuple::point(1.0, 3.0, 4.0), r.position(-1.0));
        assert_eq!(Tuple::point(4.5, 3.0, 4.0), r.position(2.5));
    }
}
