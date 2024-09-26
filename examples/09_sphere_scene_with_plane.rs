use std::f64::consts::PI;

use ray_tracer_challenge::{
    camera::Camera,
    color::{Color, Colors},
    error::RayTraceResult,
    point_light::PointLight,
    shape::{
        material::{
            pattern::{ring::RingPattern, Pattern},
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
    let wall_material = Material::new()
        .with_color(Color::new(1.0, 0.9, 0.9))
        .with_specular(0.0);

    let mut floor = Plane::new();
    floor.set_material(wall_material.clone());

    let mut back_wall = Plane::new();
    back_wall.set_material(wall_material);
    back_wall.set_transformation(
        Transformation::identity()
            .rotate_x(PI / 2.0)
            .translation(0.0, 0.0, 5.0),
    );

    let mut middle = Sphere::new();
    let mut pattern = RingPattern::new(Colors::Red.into(), Colors::White.into());
    pattern.set_transformation(
        Transformation::identity()
            .scale(0.1, 0.1, 0.1)
            .rotate_x(PI / 2.0),
    );
    middle.set_transformation(Transformation::identity().translation(-0.5, 1.0, 0.5));
    middle.set_material(
        Material::new()
            .with_pattern(pattern)
            .with_diffuse(0.7)
            .with_specular(0.3),
    );

    let mut right = Sphere::new();
    right.set_transformation(
        Transformation::identity()
            .scale(0.5, 0.5, 0.5)
            .translation(1.5, 0.5, -0.5),
    );
    right.set_material(
        Material::new()
            .with_color(Color::new(0.5, 1.0, 0.1))
            .with_diffuse(0.7)
            .with_specular(0.3),
    );

    let mut left: Sphere = Sphere::new();
    left.set_transformation(
        Transformation::identity()
            .scale(0.33, 0.33, 0.33)
            .translation(-1.5, 0.33, -0.75),
    );
    left.set_material(
        Material::new()
            .with_color(Color::new(1.0, 0.8, 0.1))
            .with_diffuse(0.7)
            .with_specular(0.3),
    );

    let mut world = World::new();
    world.add_shape(floor.into());
    world.add_shape(middle.into());
    world.add_shape(right.into());
    world.add_shape(left.into());
    world.add_shape(back_wall.into());
    world.add_light(PointLight::new(
        Tuple::point(-10.0, 10.0, -10.0),
        Colors::White.into(),
    ));

    // smaller resolution, faster rendering
    let mut camera = Camera::new(200, 150, PI / 3.0);

    // larger resolution, slower rendering
    // let mut camera = Camera::new(1200, 600, PI / 3.0);
    camera.set_transformation(Transformation::view(
        Tuple::point(0.0, 1.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    ));

    camera.render(&world).save("sphere_scene_with_planes")?;

    Ok(())
}
