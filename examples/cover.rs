use std::f64::consts::PI;

use ray_tracer_challenge::{
    camera::Camera,
    color::{Color, Colors},
    error::RayTraceResult,
    point_light::PointLight,
    shape::{cube::Cube, material::Material, plane::Plane, sphere::Sphere, Shape},
    transformation::Transformation,
    tuple::Tuple,
    world::World,
};

fn white_material() -> Material {
    Material::new()
        .with_color(Colors::White.into())
        .with_diffuse(0.7)
        .with_ambient(0.1)
        .with_specular(0.0)
        .with_reflective(0.1)
}

fn blue_material() -> Material {
    white_material().with_color(Color::new(0.537, 0.832, 0.914))
}

fn red_material() -> Material {
    white_material().with_color(Color::new(0.941, 0.322, 0.388))
}

fn purple_material() -> Material {
    white_material().with_color(Color::new(0.373, 0.404, 0.55))
}

fn standard_transform() -> Transformation {
    Transformation::identity()
        .translation(1.0, -1.0, 1.0)
        .scale(0.5, 0.5, 0.5)
}

fn large_object() -> Transformation {
    standard_transform().scale(3.5, 3.5, 3.5)
}

fn medium_object() -> Transformation {
    standard_transform().scale(3.0, 3.0, 3.0)
}

fn small_object() -> Transformation {
    standard_transform().scale(2.0, 2.0, 2.0)
}

fn main() -> RayTraceResult<()> {
    let mut world = World::new();

    let mut camera = Camera::new(200, 200, 0.785);
    camera.set_transformation(Transformation::view(
        Tuple::point(-6.0, 6.0, -10.0),
        Tuple::point(6.0, 0.0, 6.0),
        Tuple::vector(-0.45, 1.0, 0.0),
    ));

    let light1 = PointLight::new(Tuple::point(50.0, 100.0, -50.0), Colors::White.into());
    let light2 = PointLight::new(Tuple::point(-400.0, 50.0, -10.0), Color::new(0.2, 0.2, 0.2));

    world.add_light(light1);
    world.add_light(light2);

    let mut white_backdrop = Plane::new();
    white_backdrop.set_material(
        Material::new()
            .with_color(Colors::White.into())
            .with_ambient(1.0)
            .with_diffuse(0.0)
            .with_specular(0.0),
    );
    white_backdrop.set_transformation(
        Transformation::identity()
            .rotate_x(PI / 2.0)
            .translation(0.0, 0.0, 500.0),
    );
    world.add_shape(white_backdrop.into());

    let mut sphere = Sphere::new();
    sphere.set_material(
        Material::new()
            .with_color(Color::new(0.373, 0.404, 0.550))
            .with_diffuse(0.2)
            .with_ambient(0.0)
            .with_specular(1.0)
            .with_shininess(200.0)
            .with_reflective(0.7)
            .with_transparency(0.7)
            .with_refractive_index(1.5),
    );
    sphere.set_transformation(large_object());
    world.add_shape(sphere.into());

    let mut cube = Cube::new();
    cube.set_material(white_material());
    cube.set_transformation(medium_object().translation(4.0, 0.0, 0.0));
    world.add_shape(cube.into());

    let mut cube = Cube::new();
    cube.set_material(blue_material());
    cube.set_transformation(large_object().translation(8.5, 1.5, -0.5));
    world.add_shape(cube.into());

    let mut cube = Cube::new();
    cube.set_material(red_material());
    cube.set_transformation(large_object().translation(0.0, 0.0, 4.0));
    world.add_shape(cube.into());

    let mut cube = Cube::new();
    cube.set_material(white_material());
    cube.set_transformation(large_object().translation(4.0, 0.0, 4.0));
    world.add_shape(cube.into());

    let mut cube = Cube::new();
    cube.set_material(purple_material());
    cube.set_transformation(medium_object().translation(7.5, 0.5, 4.0));
    world.add_shape(cube.into());

    let mut cube = Cube::new();
    cube.set_material(white_material());
    cube.set_transformation(medium_object().translation(-0.25, 0.25, 8.0));
    world.add_shape(cube.into());

    let mut cube = Cube::new();
    cube.set_material(blue_material());
    cube.set_transformation(large_object().translation(4.0, 1.0, 7.5));
    world.add_shape(cube.into());

    let mut cube = Cube::new();
    cube.set_material(red_material());
    cube.set_transformation(medium_object().translation(10.0, 2.0, 7.5));
    world.add_shape(cube.into());

    let mut cube = Cube::new();
    cube.set_material(white_material());
    cube.set_transformation(small_object().translation(8.0, 2.0, 12.0));
    world.add_shape(cube.into());

    let mut cube = Cube::new();
    cube.set_material(white_material());
    cube.set_transformation(small_object().translation(20.0, 1.0, 9.0));
    world.add_shape(cube.into());

    let mut cube = Cube::new();
    cube.set_material(blue_material());
    cube.set_transformation(large_object().translation(-0.5, -5.0, 0.25));
    world.add_shape(cube.into());

    let mut cube = Cube::new();
    cube.set_material(red_material());
    cube.set_transformation(large_object().translation(4.0, -4.0, 0.0));
    world.add_shape(cube.into());

    let mut cube = Cube::new();
    cube.set_material(white_material());
    cube.set_transformation(large_object().translation(8.5, -4.0, 0.0));
    world.add_shape(cube.into());

    let mut cube = Cube::new();
    cube.set_material(white_material());
    cube.set_transformation(large_object().translation(0.0, -4.0, 4.0));
    world.add_shape(cube.into());

    let mut cube = Cube::new();
    cube.set_material(purple_material());
    cube.set_transformation(large_object().translation(-0.5, -4.6, 8.0));
    world.add_shape(cube.into());

    let mut cube = Cube::new();
    cube.set_material(white_material());
    cube.set_transformation(large_object().translation(0.0, -8.0, 4.0));
    world.add_shape(cube.into());

    let mut cube = Cube::new();
    cube.set_material(white_material());
    cube.set_transformation(large_object().translation(-0.5, -8.5, 8.0));
    world.add_shape(cube.into());

    camera.render(&world).save("cover")
}
