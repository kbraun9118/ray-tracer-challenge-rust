use std::{
    cmp::{max, min},
    ops::{Add, Mul, Sub},
};

use crate::tuple::Tuple;

#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

pub enum Colors {
    Red,
    White,
    Black,
    Blue,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }

    pub fn red(&self) -> f64 {
        self.red
    }

    pub fn green(&self) -> f64 {
        self.green
    }

    pub fn blue(&self) -> f64 {
        self.blue
    }

    pub fn to_ppm(self) -> (u8, u8, u8) {
        let scaled = self * 255.0;
        (
            max(0, min(255, scaled.red().round() as u8)),
            max(0, min(255, scaled.green().round() as u8)),
            max(0, min(255, scaled.blue().round() as u8)),
        )
    }
}

impl From<Colors> for Color {
    fn from(value: Colors) -> Self {
        use Colors::*;

        let (red, green, blue) = match value {
            Red => (1.0, 0.0, 0.0),
            White => (1.0, 1.0, 1.0),
            Black => (0.0, 0.0, 0.0),
            Blue => (0.0, 0.0, 1.0)
        };

        Self::new(red, green, blue)
    }
}

impl From<Tuple> for Color {
    fn from(value: Tuple) -> Self {
        Color {
            red: value.x(),
            green: value.y(),
            blue: value.z(),
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        Tuple::from(*self) == Tuple::from(*other)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        (Tuple::from(self) + Tuple::from(rhs)).into()
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        (Tuple::from(self) - Tuple::from(rhs)).into()
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        (Tuple::from(self) * rhs).into()
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::eq_f64;

    #[test]
    fn colors_are_red_green_blue() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert!(eq_f64(-0.5, c.red));
        assert!(eq_f64(0.4, c.green));
        assert!(eq_f64(1.7, c.blue));
    }

    #[test]
    fn ppm_converts_to_ppm() {
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        assert_eq!((255, 0, 0), c1.to_ppm());
        assert_eq!((0, 128, 0), c2.to_ppm());
        assert_eq!((0, 0, 255), c3.to_ppm());
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(1.6, 0.7, 1.0);

        assert_eq!(expected, c1 + c2);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(0.2, 0.5, 0.5);

        assert_eq!(expected, c1 - c2);
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        let expected = Color::new(0.4, 0.6, 0.8);

        assert_eq!(expected, c * 2.0);
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let expected = Color::new(0.9, 0.2, 0.04);

        assert_eq!(expected, c1 * c2);
    }
}
