use std::ops::Mul;

use crate::{intersection::ray::Ray, matrix::Matrix, tuple::Tuple};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Transformation {
    matrix: Matrix,
}

impl Transformation {
    pub fn identity() -> Self {
        Self {
            matrix: Matrix::identity(4),
        }
    }

    pub fn inverse(&self) -> Option<Self> {
        self.matrix.inverse().map(|matrix| Self { matrix })
    }

    pub fn translation(&self, x: f64, y: f64, z: f64) -> Self {
        let mut m = Matrix::identity(4);
        m[(0, 3)] = x;
        m[(1, 3)] = y;
        m[(2, 3)] = z;

        Self {
            matrix: &m * &self.matrix,
        }
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Self {
        let mut m = Matrix::identity(4);
        m[(0, 0)] = x;
        m[(1, 1)] = y;
        m[(2, 2)] = z;

        Self {
            matrix: &m * &self.matrix,
        }
    }

    pub fn rotate_x(&self, radians: f64) -> Self {
        let mut m = Matrix::identity(4);
        m[(1, 1)] = radians.cos();
        m[(2, 2)] = radians.cos();
        m[(1, 2)] = -radians.sin();
        m[(2, 1)] = radians.sin();

        Self {
            matrix: &m * &self.matrix,
        }
    }

    pub fn rotate_y(&self, radians: f64) -> Self {
        let mut m = Matrix::identity(4);
        m[(0, 0)] = radians.cos();
        m[(2, 2)] = radians.cos();
        m[(0, 2)] = radians.sin();
        m[(2, 0)] = -radians.sin();

        Self {
            matrix: &m * &self.matrix,
        }
    }

    pub fn rotate_z(&self, radians: f64) -> Self {
        let mut m = Matrix::identity(4);
        m[(0, 0)] = radians.cos();
        m[(1, 1)] = radians.cos();
        m[(0, 1)] = -radians.sin();
        m[(1, 0)] = radians.sin();

        Self {
            matrix: &m * &self.matrix,
        }
    }

    pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let mut m = Matrix::identity(4);
        m[(0, 1)] = xy;
        m[(0, 2)] = xz;
        m[(1, 0)] = yx;
        m[(1, 2)] = yz;
        m[(2, 0)] = zx;
        m[(2, 1)] = zy;

        Self {
            matrix: &m * &self.matrix,
        }
    }

    pub fn transpose(&self) -> Self {
        Self {
            matrix: self.matrix.transpose(),
        }
    }

    pub fn view(from: Tuple, to: Tuple, up: Tuple) -> Self {
        let forward = (to - from).normalize();
        let left = forward ^ up.normalize();
        let true_up = left ^ forward;

        let orientation = Matrix::from(vec![
            vec![left.x(), left.y(), left.z(), 0.0],
            vec![true_up.x(), true_up.y(), true_up.z(), 0.0],
            vec![-forward.x(), -forward.y(), -forward.z(), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        Self {
            matrix: &orientation * &Self::identity().translation(-from.x(), -from.y(), -from.z()).matrix,
        }
    }
}

impl Mul<Tuple> for &Transformation {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        &self.matrix * rhs
    }
}

impl Mul<Tuple> for Transformation {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Ray> for Transformation {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Self::Output {
        Ray::new(&self * rhs.origin(), &self * rhs.direction())
    }
}

#[cfg(test)]
mod tests {
    use std::{f64::consts::PI, vec};

    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Transformation::identity().translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(Tuple::point(2.0, 1.0, 7.0), transform * p)
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Transformation::identity().translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(
            Tuple::point(-8.0, 7.0, 3.0),
            transform.inverse().unwrap() * p
        );
    }

    #[test]
    fn trnaslation_does_not_effect_vector() {
        let transform = Transformation::identity().translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(Tuple::vector(-3.0, 4.0, 5.0), transform * v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = Transformation::identity().scale(2.0, 3.0, 4.0);
        let a = Tuple::point(-4.0, 6.0, 8.0);

        assert_eq!(Tuple::point(-8.0, 18.0, 32.0), transform * a);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = Transformation::identity().scale(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(Tuple::vector(-8.0, 18.0, 32.0), transform * v);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Transformation::identity().scale(2.0, 3.0, 4.0);
        let inverse = transform.inverse().unwrap();
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(Tuple::vector(-2.0, 2.0, 2.0), inverse * v);
    }

    #[test]
    fn scaling_by_a_negative_is_reflection() {
        let transform = Transformation::identity().scale(-1.0, 1.0, 1.0);
        let v = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(Tuple::vector(-2.0, 3.0, 4.0), transform * v);
    }

    #[test]
    fn rotating_around_the_x_axis() {
        let half_quarter = Transformation::identity().rotate_x(PI / 4.0);
        let quarter = Transformation::identity().rotate_x(PI / 2.0);
        let p = Tuple::point(0.0, 1.0, 0.0);

        assert_eq!(
            Tuple::point(0.0, 2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0),
            half_quarter * p
        );
        assert_eq!(Tuple::point(0.0, 0.0, 1.0), quarter * p);
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let half_quarter = Transformation::identity().rotate_x(PI / 4.0);
        let inverse = half_quarter.inverse().unwrap();
        let p = Tuple::point(0.0, 1.0, 0.0);

        assert_eq!(
            Tuple::point(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0),
            inverse * p
        )
    }

    #[test]
    fn rotating_around_the_y_axis() {
        let half_quarter = Transformation::identity().rotate_y(PI / 4.0);
        let quarter = Transformation::identity().rotate_y(PI / 2.0);
        let p = Tuple::point(0.0, 0.0, 1.0);

        assert_eq!(
            Tuple::point(2f64.sqrt() / 2.0, 0.0, 2f64.sqrt() / 2.0),
            half_quarter * p
        );
        assert_eq!(Tuple::point(1.0, 0.0, 0.0), quarter * p);
    }

    #[test]
    fn rotating_around_the_z_axis() {
        let half_quarter = Transformation::identity().rotate_z(PI / 4.0);
        let quarter = Transformation::identity().rotate_z(PI / 2.0);
        let p = Tuple::point(0.0, 1.0, 0.0);

        assert_eq!(
            Tuple::point(-2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0, 0.0),
            half_quarter * p
        );
        assert_eq!(Tuple::point(-1.0, 0.0, 0.0), quarter * p);
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transformation = Transformation::identity().shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let expected = Tuple::point(5.0, 3.0, 4.0);

        assert_eq!(expected, transformation * p);
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transformation = Transformation::identity().shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let expected = Tuple::point(6.0, 3.0, 4.0);

        assert_eq!(expected, transformation * p);
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transformation = Transformation::identity().shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let expected = Tuple::point(2.0, 5.0, 4.0);

        assert_eq!(expected, transformation * p);
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transformation = Transformation::identity().shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let expected = Tuple::point(2.0, 7.0, 4.0);

        assert_eq!(expected, transformation * p);
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transformation = Transformation::identity().shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let expected = Tuple::point(2.0, 3.0, 6.0);

        assert_eq!(expected, transformation * p);
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transformation = Transformation::identity().shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let expected = Tuple::point(2.0, 3.0, 7.0);

        assert_eq!(expected, transformation * p);
    }

    #[test]
    fn individual_tranformations_applied_in_squence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Transformation::identity().rotate_x(PI / 2.0);
        let b = Transformation::identity().scale(5.0, 5.0, 5.0);
        let c = Transformation::identity().translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(Tuple::point(1.0, -1.0, 0.0), p2);

        let p3 = b * p2;
        assert_eq!(Tuple::point(5.0, -5.0, 0.0), p3);

        let p4 = c * p3;
        assert_eq!(Tuple::point(15.0, 0.0, 7.0), p4);
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let tranformation = Transformation::identity()
            .rotate_x(PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translation(10.0, 5.0, 7.0);

        assert_eq!(Tuple::point(15.0, 0.0, 7.0), tranformation * p)
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = Transformation::identity().translation(3.0, 4.0, 5.0);

        let r2 = m * r;

        assert_eq!(Tuple::point(4.0, 6.0, 8.0), r2.origin());
        assert_eq!(Tuple::vector(0.0, 1.0, 0.0), r2.direction());
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = Transformation::identity().scale(2.0, 3.0, 4.0);

        let r2 = m * r;

        assert_eq!(Tuple::point(2.0, 6.0, 12.0), r2.origin());
        assert_eq!(Tuple::vector(0.0, 3.0, 0.0), r2.direction());
    }

    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        let from = Tuple::origin();
        let to = Tuple::point(0.0, 0.0, -1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = Transformation::view(from, to, up);

        assert_eq!(Transformation::identity(), t);
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Tuple::origin();
        let to = Tuple::point(0.0, 0.0, 1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = Transformation::view(from, to, up);

        assert_eq!(Transformation::identity().scale(-1.0, 1.0, -1.0), t);
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = Tuple::point(0.0, 0.0, 8.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = Transformation::view(from, to, up);

        assert_eq!(Transformation::identity().translation(0.0, 0.0, -8.0), t);
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = Tuple::point(1.0, 3.0, 2.0);
        let to = Tuple::point(4.0, -2.0, 8.0);
        let up = Tuple::vector(1.0, 1.0, 0.0);

        let t = Transformation::view(from, to, up);

        let expected = Matrix::from(vec![
            vec![-0.50709, 0.50709, 0.67612, -2.36643],
            vec![0.76772, 0.60609, 0.12122, -2.82843],
            vec![-0.35857, 0.59761, -0.71714, 0.00000],
            vec![0.00000, 0.00000, 0.00000, 1.00000],
        ]);

        assert_eq!(expected, t.matrix);
    }
}
