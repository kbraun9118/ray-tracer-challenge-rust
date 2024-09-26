use ray_tracer_challenge::{
    camera::Camera,
    color::{Color, Colors},
    error::RayTraceResult,
    point_light::PointLight,
    shape::{
        material::{
            pattern::{checker::CheckerPattern, Pattern},
            Material,
        },
        plane::Plane,
        sphere::Sphere,
        Shape,
    },
    transformation::Transformation,
    tuple::Tuple,
    world::World,
};

fn main() -> RayTraceResult<()> {
    let mut camera = Camera::new(600, 600, 0.45);
    camera.set_transformation(Transformation::view(
        Tuple::point(0.0, 0.0, -5.0),
        Tuple::point(0.0, 0.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    ));

    let mut world = World::new();
    world.add_light(PointLight::new(
        Tuple::point(2.0, 10.0, -5.0),
        Color::new(0.9, 0.9, 0.9),
    ));

    let mut plane = Plane::new();
    plane.set_transformation(
        Transformation::identity()
            .rotate_x(1.5708)
            .translation(0.0, 0.0, 10.0),
    );
    plane.set_material(
        Material::new()
            .with_pattern(CheckerPattern::new(
                Color::new(0.15, 0.15, 0.15),
                Color::new(0.85, 0.85, 0.85),
            ))
            .with_ambient(0.8)
            .with_diffuse(0.2)
            .with_specular(0.0),
    );
    world.add_shape(plane.into());

    let mut sphere = Sphere::new();
    sphere.set_material(
        Material::new()
            .with_color(Colors::White.into())
            .with_ambient(0.0)
            .with_diffuse(0.0)
            .with_specular(0.9)
            .with_shininess(300.0)
            .with_reflective(0.9)
            .with_transparency(0.9)
            .with_refractive_index(1.5),
    );
    world.add_shape(sphere.into());

    let mut sphere = Sphere::new();
    sphere.set_transformation(Transformation::identity().scale(0.5, 0.5, 0.5));
    sphere.set_material(
        Material::new()
            .with_color(Colors::White.into())
            .with_ambient(0.0)
            .with_diffuse(0.0)
            .with_specular(0.9)
            .with_shininess(300.0)
            .with_reflective(0.9)
            .with_transparency(0.9)
            .with_refractive_index(1.0000034),
    );
    world.add_shape(sphere.into());

    camera.render(&world).save("fresnal")
}
