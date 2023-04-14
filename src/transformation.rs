use crate::matrix::Matrix;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = Matrix::identity(4);
    m[(0, 3)] = x;
    m[(1, 3)] = y;
    m[(2, 3)] = z;
    m
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(Tuple::point(2.0, 1.0, 7.0), transform * p)
    }
}