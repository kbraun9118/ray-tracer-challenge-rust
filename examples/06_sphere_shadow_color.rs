use ray_tracer_challenge::{
    canvas::Canvas,
    color::{Color, Colors},
    error::RayTraceResult,
    intersection::ray::Ray,
    point_light::PointLight,
    shape::{material::Material, sphere::Sphere, Shape, ShapeContainer},
    transformation::Transformation,
    tuple::Tuple,
};

fn main() -> RayTraceResult<()> {
    let mut c = Canvas::new(400, 400);
    let material = Material::new().with_color(Color::new(1.0, 0.2, 1.0));
    let mut sphere = Sphere::new();

    sphere.set_material(material);
    sphere.set_transformation(
        Transformation::identity()
            .scale(50.0, 50.0, 50.0)
            .translation(200.0, 200.0, -300.0),
    );
    let sphere = ShapeContainer::from(sphere);

    let light_position = Tuple::point(-100.0, -100.0, -600.0);
    let light_color = Colors::White.into();
    let light = PointLight::new(light_position, light_color);

    for y in 0..400 {
        for x in 0..400 {
            let r = Ray::new(
                Tuple::point(200.0, 200.0, -500.0),
                Tuple::vector(-200.0 + x as f64, -200.0 + y as f64, 500.0).normalize(),
            );

            let intersections = r.intersections(sphere.clone());

            c[(x, y)] = if let Some(hit) = intersections.hit() {
                let point = r.position(hit.t());
                let normal = hit
                    .object()
                    .write()
                    .unwrap()
                    .normal_at(sphere.id(), point, hit.clone())
                    .unwrap();
                let eye = -r.direction();
                hit.object()
                    .write()
                    .unwrap()
                    .material(hit.object_id())
                    .unwrap()
                    .lighting(hit.object().clone(), light, point, eye, normal, false)
            } else {
                Colors::Black.into()
            };
        }
    }

    c.save("spehere_shadow_color")?;

    Ok(())
}
