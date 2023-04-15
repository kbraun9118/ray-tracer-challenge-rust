use crate::matrix::Matrix;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = Matrix::identity(4);
    m[(0, 3)] = x;
    m[(1, 3)] = y;
    m[(2, 3)] = z;
    m
}

pub fn scale(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = Matrix::identity(4);
    m[(0, 0)] = x;
    m[(1, 1)] = y;
    m[(2, 2)] = z;
    m
}

pub fn rotate_x(radians: f64) -> Matrix {
    let mut m = Matrix::identity(4);
    m[(1, 1)] = radians.cos();
    m[(2, 2)] = radians.cos();
    m[(1, 2)] = -radians.sin();
    m[(2, 1)] = radians.sin();
    m
}

pub fn rotate_y(radians: f64) -> Matrix {
    let mut m = Matrix::identity(4);
    m[(0, 0)] = radians.cos();
    m[(2, 2)] = radians.cos();
    m[(0, 2)] = radians.sin();
    m[(2, 0)] = -radians.sin();
    m
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(Tuple::point(2.0, 1.0, 7.0), transform * p)
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(
            Tuple::point(-8.0, 7.0, 3.0),
            transform.inverse().unwrap() * p
        );
    }

    #[test]
    fn trnaslation_does_not_effect_vector() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(Tuple::vector(-3.0, 4.0, 5.0), transform * v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = scale(2.0, 3.0, 4.0);
        let a = Tuple::point(-4.0, 6.0, 8.0);

        assert_eq!(Tuple::point(-8.0, 18.0, 32.0), transform * a);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = scale(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(Tuple::vector(-8.0, 18.0, 32.0), transform * v);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = scale(2.0, 3.0, 4.0);
        let inverse = transform.inverse().unwrap();
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(Tuple::vector(-2.0, 2.0, 2.0), inverse * v);
    }

    #[test]
    fn scaling_by_a_negative_is_reflection() {
        let transform = scale(-1.0, 1.0, 1.0);
        let v = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(Tuple::vector(-2.0, 3.0, 4.0), transform * v);
    }

    #[test]
    fn rotating_around_the_x_axis() {
        let half_quarter = rotate_x(PI / 4.0);
        let quarter = rotate_x(PI / 2.0);
        let p = Tuple::point(0.0, 1.0, 0.0);

        assert_eq!(
            Tuple::point(0.0, 2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0),
            half_quarter * p
        );
        assert_eq!(Tuple::point(0.0, 0.0, 1.0), quarter * p);
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let half_quarter = rotate_x(PI / 4.0);
        let inverse = half_quarter.inverse().unwrap();
        let p = Tuple::point(0.0, 1.0, 0.0);

        assert_eq!(
            Tuple::point(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0),
            inverse * p
        )
    }

    #[test]
    fn rotating_around_the_y_axis() {
        let half_quarter = rotate_y(PI / 4.0);
        let quarter = rotate_y(PI / 2.0);
        let p = Tuple::point(0.0, 0.0, 1.0);

        assert_eq!(
            Tuple::point(2f64.sqrt() / 2.0, 0.0, 2f64.sqrt() / 2.0),
            half_quarter * p
        );
        assert_eq!(Tuple::point(1.0, 0.0, 0.0), quarter * p);
    }
}
