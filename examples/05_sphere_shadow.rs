use ray_tracer_challenge::{
    canvas::Canvas,
    color::Colors,
    error::RayTraceResult,
    intersection::{
        ray::Ray,
        shape::{sphere::Sphere, Shape},
    },
    transformation::Transformation,
    tuple::Tuple,
};

fn main() -> RayTraceResult<()> {
    let mut c = Canvas::new(400, 400);
    let mut sphere = Sphere::new();
    sphere.set_transformation(
        Transformation::identity()
            .scale(50.0, 50.0, 50.0)
            .translation(200.0, 200.0, -300.0),
    );

    for y in 0..400 {
        for x in 0..400 {
            let r = Ray::new(
                Tuple::point(200.0, 200.0, -500.0),
                Tuple::vector(-200.0 + x as f64, -200.0 + y as f64, 500.0),
            );

            c[(x, y)] = if sphere.intersects(r).len() > 0 {
                Colors::Red.into()
            } else {
                Colors::Black.into()
            };
        }
    }

    c.save("spehere_shadow")?;

    Ok(())
}
