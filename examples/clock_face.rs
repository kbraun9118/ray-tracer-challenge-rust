use std::f64::consts::PI;

use ray_tracer_challenge::{
    canvas::Canvas, color::Color, error::RayTraceResult, transformation::Transformation,
    tuple::Tuple,
};

fn main() -> RayTraceResult<()> {
    let mut c = Canvas::fill_with(100, 100, Color::white());

    for i in 0..12 {
        let point = Tuple::point(0.0, 45.0, 0.0);
        let transformation = Transformation::identity()
            .rotate_z(i as f64 * ((2.0 * PI) / 12.0))
            .translation(50.0, 50.0, 0.0);
        let point = transformation * point;

        c[point] = Color::red();
    }

    c.save("clock")?;

    Ok(())
}
