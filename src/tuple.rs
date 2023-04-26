use std::ops::{Add, BitXor, Div, Mul, Neg, Sub};

use crate::{color::Color, util::eq_f64};

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn w(&self) -> f64 {
        self.w
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, 1.0)
    }

    pub fn origin() -> Self {
        Self::point(0.0, 0.0, 0.0)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        *self / self.magnitude()
    }

    pub fn reflect(&self, normal: Tuple) -> Tuple {
        *self - normal * 2.0 * (*self * normal)
    }

    pub fn as_vector(&mut self) {
        self.w = 0.0;
    }
}

impl From<Color> for Tuple {
    fn from(value: Color) -> Self {
        Tuple::new(value.red(), value.green(), value.blue(), 0.0)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        eq_f64(self.x, other.x)
            && eq_f64(self.y, other.y)
            && eq_f64(self.z, other.z)
            && eq_f64(self.w, other.w)
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl Mul for Tuple {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w;
    }
}

impl BitXor for Tuple {
    type Output = Tuple;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Tuple::vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::util::eq_f64;

    use super::*;

    #[test]
    fn tuple_with_w_0_is_a_point() {
        let a = Tuple::point(4.3, -4.2, 3.1);
        assert!(eq_f64(a.x, 4.3));
        assert!(eq_f64(a.y, -4.2));
        assert!(eq_f64(a.z, 3.1));
        assert!(eq_f64(a.w, 1.0));
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_with_w_0_is_a_vector() {
        let a = Tuple::vector(4.3, -4.2, 3.1);
        assert!(eq_f64(a.x, 4.3));
        assert!(eq_f64(a.y, -4.2));
        assert!(eq_f64(a.z, 3.1));
        assert!(eq_f64(a.w, 0.0));
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn partial_eq() {
        let a = Tuple::point(4.3, -4.2, 3.1);
        let b = Tuple::point(4.3, -4.2, 3.1);
        let c = Tuple::vector(4.3, -4.2, 3.1);

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let expected = Tuple::new(1.0, 1.0, 6.0, 1.0);

        assert_eq!(expected, a1 + a2);
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);
        let expected = Tuple::vector(-2.0, -4.0, -6.0);

        assert_eq!(expected, p1 - p2);
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);
        let expected = Tuple::point(-2.0, -4.0, -6.0);

        assert_eq!(expected, p - v);
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);
        let expected = Tuple::vector(-2.0, -4.0, -6.0);

        assert_eq!(expected, v1 - v2);
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple::new(1., -2., 3., -4.);
        let expected = Tuple::new(-1., 2., -3., 4.);

        assert_eq!(expected, -a)
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::new(1., -2., 3., -4.);
        let expected = Tuple::new(3.5, -7., 10.5, -14.);

        assert_eq!(expected, a * 3.5)
    }

    #[test]
    fn divide_a_tuple_by_a_scalar() {
        let a = Tuple::new(1., -2., 3., -4.);
        let expected = Tuple::new(0.5, -1., 1.5, -2.);

        assert_eq!(expected, a / 2.);
    }

    #[test]
    fn magnitude_of_vectors() {
        assert!(eq_f64(Tuple::vector(1., 0., 0.).magnitude(), 1.));
        assert!(eq_f64(Tuple::vector(0., 1., 0.).magnitude(), 1.));
        assert!(eq_f64(Tuple::vector(0., 0., 1.).magnitude(), 1.));
        assert!(eq_f64(
            Tuple::vector(1., 2., 3.).magnitude(),
            14.0f64.sqrt()
        ));
        assert!(eq_f64(
            Tuple::vector(-1., -2., -3.).magnitude(),
            14.0f64.sqrt()
        ));
    }

    #[test]
    fn normalizing_vectors() {
        assert_eq!(
            Tuple::vector(1., 0., 0.),
            Tuple::vector(4., 0., 0.).normalize()
        );
        let sqrt14 = 14f64.sqrt();
        let normalized = Tuple::vector(1., 2., 3.).normalize();
        assert_eq!(
            Tuple::vector(1. / sqrt14, 2. / sqrt14, 3. / sqrt14),
            normalized
        );
        assert!(eq_f64(1., normalized.magnitude()));
    }

    #[test]
    fn the_dot_product_of_two_tuples() {
        let a = Tuple::vector(1., 2., 3.);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert!(eq_f64(20.0, a * b));
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        let a_cross_b = Tuple::vector(-1.0, 2.0, -1.0);
        let b_cross_a = Tuple::vector(1.0, -2.0, 1.0);

        assert_eq!(a_cross_b, a ^ b);
        assert_eq!(b_cross_a, b ^ a);
    }

    #[test]
    fn reflecting_a_vector_approaching_at_45() {
        let v = Tuple::vector(1.0, -1.0, 0.0);
        let n = Tuple::vector(0.0, 1.0, 0.0);

        let r = v.reflect(n);

        assert_eq!(Tuple::vector(1.0, 1.0, 0.0), r);
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = Tuple::vector(0.0, -1.0, 0.0);
        let n = Tuple::vector(2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0, 0.0);

        let r = v.reflect(n);

        assert_eq!(Tuple::vector(1.0, 0.0, 0.0), r);
    }


}
