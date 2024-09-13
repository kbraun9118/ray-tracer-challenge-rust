use std::f64::consts::PI;

use ray_tracer_challenge::{
    camera::Camera,
    color::{Color, Colors},
    error::RayTraceResult,
    point_light::PointLight,
    shape::{material::Material, sphere::Sphere, Shape},
    transformation::Transformation,
    tuple::Tuple,
    world::World,
};

fn main() -> RayTraceResult<()> {
    let wall_material = Material::new()
        .with_color(Color::new(1.0, 0.9, 0.9))
        .with_specular(0.0);

    let mut floor = Sphere::new();
    floor.set_transformation(Transformation::identity().scale(10.0, 0.01, 10.0));
    floor.set_material(wall_material.clone());

    let mut left_wall = Sphere::new();
    left_wall.set_transformation(
        Transformation::identity()
            .scale(10.0, 0.01, 10.0)
            .rotate_x(PI / 2.0)
            .rotate_y(-PI / 4.0)
            .translation(0.0, 0.0, 5.0),
    );
    left_wall.set_material(wall_material.clone());

    let mut right_wall = Sphere::new();
    right_wall.set_transformation(
        Transformation::identity()
            .scale(10.0, 0.01, 10.0)
            .rotate_x(PI / 2.0)
            .rotate_y(PI / 4.0)
            .translation(0.0, 0.0, 5.0),
    );
    right_wall.set_material(wall_material);

    let mut middle = Sphere::new();
    middle.set_transformation(Transformation::identity().translation(-0.5, 1.0, 0.5));
    middle.set_material(
        Material::new()
            .with_color(Color::new(0.1, 1.0, 0.5))
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
    world.add_shape(left_wall.into());
    world.add_shape(right_wall.into());
    world.add_shape(middle.into());
    world.add_shape(right.into());
    world.add_shape(left.into());
    world.set_light(PointLight::new(
        Tuple::point(-10.0, 10.0, -10.0),
        Colors::White.into(),
    ));

    // smaller resolution, faster rendering
    let mut camera = Camera::new(200, 100, PI / 3.0);

    // larger resolution, slower rendering
    // let mut camera = Camera::new(1200 * 2, 600 * 2, PI / 3.0);
    camera.set_transformation(Transformation::view(
        Tuple::point(0.0, 1.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    ));

    camera.render(&world).save("sphere_scene")?;

    Ok(())
}
