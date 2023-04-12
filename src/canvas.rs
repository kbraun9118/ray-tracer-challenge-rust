use std::{
    fs::File,
    io::Write,
    ops::{Index, IndexMut},
};

use crate::{color::Color, error::RayTraceResult};

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

    fn ppm_header(&self) -> String {
        format!("P3\n{} {}\n255", self.width(), self.height())
    }

    fn ppm_body(&self) -> String {
        let mut body = String::from("");
        for y in 0..self.height() {
            let mut colors = vec![];
            let mut line = String::default();
            for x in 0..self.width() {
                let (red, green, blue) = self[(x, y)].to_ppm();
                colors.push(red);
                colors.push(green);
                colors.push(blue);
            }
            for color in colors {
                let color = color.to_string();
                if color.len() + line.len() > 69 {
                    body.push_str(line.trim_end());
                    body.push_str("\n");
                    line = String::default();
                }

                line.push_str(&color);
                line.push_str(" ")
            }
            body.push_str(line.trim_end());
            body.push_str("\n")
        }

        body
    }

    pub fn save(self, filename: &str) -> RayTraceResult<()> {
        let mut filename = filename.to_owned();

        if !filename.ends_with(".ppm") {
            filename = format!("{}.ppm", filename);
        }

        let mut file = File::create(filename)?;
        let contents = format!("{}\n{}", self.ppm_header(), self.ppm_body());
        file.write_all(contents.as_bytes())?;

        Ok(())
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

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.ppm_header();
        let expected = "P3\n5 3\n255";
        assert_eq!(expected, ppm)
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        c[(0, 0)] = Color::new(1.5, 0.0, 0.0);
        c[(2, 1)] = Color::new(0.0, 0.5, 0.0);
        c[(4, 2)] = Color::new(-0.5, 0.0, 1.0);
        let expected = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";

        assert_eq!(expected, c.ppm_body());
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        for y in 0..2 {
            for x in 0..10 {
                c[(x, y)] = Color::new(1.0, 0.8, 0.6);
            }
        }
        let expected = r#"255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
"#;
        assert_eq!(expected, c.ppm_body());
    }
}
