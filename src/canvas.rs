use std::ops::{Index, IndexMut};

use crate::color::Color;

pub struct Canvas {
    width: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            pixels: vec![Color::default(); width * height],
        }
    }

    pub fn height(&self) -> usize {
        self.pixels.len() / self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = Color;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pixels[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[y * self.width + x]
    }
}

impl IntoIterator for Canvas {
    type Item = Color;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        let width = c.width();
        let height = c.height();
        assert_eq!(10, width);
        assert_eq!(20, height);
        let black = Color::new(0.0, 0.0, 0.0);

        for p in c {
            assert_eq!(black, p);
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c[(2, 3)] = red;
        assert_eq!(red, c[(2, 3)]);
    }
}
