use indicatif::ProgressBar;

use crate::{
    canvas::Canvas, intersection::ray::Ray, transformation::Transformation, tuple::Tuple,
    util::eq_f64, world::World,
};

pub struct Camera {
    h_size: f64,
    v_size: f64,
    transform: Transformation,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(h_size: f64, v_size: f64, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = h_size / v_size;
        let (half_width, half_height) = if eq_f64(1.0, aspect) || aspect > 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        Self {
            h_size,
            v_size,
            transform: Transformation::identity(),
            half_width,
            half_height,
            pixel_size: (half_width * 2.0) / h_size,
        }
    }

    pub fn set_transformation(&mut self, transformation: Transformation) {
        self.transform = transformation;
    }

    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let x_offset = (px as f64 + 0.5) * self.pixel_size;
        let y_offset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let transform_invese = self.transform.inverse().unwrap();

        let pixel = transform_invese.clone() * Tuple::point(world_x, world_y, -1.0);
        let origin = transform_invese * Tuple::origin();
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.h_size as usize, self.v_size as usize);
        let pb = ProgressBar::new((self.v_size * self.h_size ) as u64);

        for y in 0..self.v_size as usize {
            for x in 0..self.h_size as usize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);
                image[(x, y)] = color;
                pb.inc(1);
            }
        }
        pb.finish_with_message("Rendering complete");

        image
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{color::Color, tuple::Tuple};

    use super::*;

    #[test]
    fn constructing_a_camera() {
        let c = Camera::new(160.0, 120.0, PI / 2.0);

        assert_eq!(160.0, c.h_size);
        assert_eq!(120.0, c.v_size);
        assert_eq!(Transformation::identity(), c.transform);
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200.0, 125.0, PI / 2.0);
        assert!(eq_f64(0.01, c.pixel_size));
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125.0, 200.0, PI / 2.0);
        assert!(eq_f64(0.01, c.pixel_size));
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201.0, 101.0, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(Tuple::origin(), r.origin());
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), r.direction());
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201.0, 101.0, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(Tuple::origin(), r.origin());
        assert_eq!(Tuple::vector(0.66519, 0.33259, -0.66851), r.direction());
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201.0, 101.0, PI / 2.0);
        c.set_transformation(
            Transformation::identity()
                .translation(0.0, -2.0, 5.0)
                .rotate_y(PI / 4.0),
        );
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(Tuple::point(0.0, 2.0, -5.0), r.origin());
        assert_eq!(
            Tuple::vector(2f64.sqrt() / 2.0, 0.0, -2f64.sqrt() / 2.0),
            r.direction()
        );
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World::default();
        let mut c = Camera::new(11.0, 11.0, PI / 2.0);
        let from = Tuple::point(0.0, 0.0, -5.0);
        let to = Tuple::origin();
        let up = Tuple::vector(0.0, 1.0, 0.0);

        c.set_transformation(Transformation::view(from, to, up));

        let image = c.render(&w);

        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), image[(5, 5)])
    }
}
