use core::f64;
use std::f64::consts::PI;

use ray_tracer_challenge::{
    camera::Camera,
    color::Colors,
    error::RayTraceResult,
    obj::OBJParser,
    point_light::PointLight,
    shape::{
        group::GroupContainer,
        material::{pattern::checker::CheckerPattern, Material},
        plane::Plane,
        Shape,
    },
    transformation::Transformation,
    tuple::Tuple,
    world::World,
};

fn teapot() -> RayTraceResult<GroupContainer> {
    OBJParser::parse_file("./examples/objs/15_teapot_low_res.obj").map(|p| p.as_group())
}

fn main() -> RayTraceResult<()> {
    let mut world = World::new();
    let teapot = teapot()?;
    let scale = 1.0 / 5.0;
    teapot.write().unwrap().set_transformation(
        Transformation::identity()
            .scale(scale, scale, scale)
            .rotate_x(-PI / 2.0),
    );

    world.add_shape(teapot.into());

    world.add_light(PointLight::new(
        Tuple::point(-10.0, 10.0, -10.0),
        Colors::White.into(),
    ));
    let mut back_wall = Plane::new();
    back_wall.set_transformation(
        Transformation::identity()
            .rotate_x(f64::consts::PI / 2.0)
            .translation(0.0, 0.0, 5.0),
    );
    back_wall.set_material(Material::new().with_pattern(CheckerPattern::new(
        Colors::Black.into(),
        Colors::Purple.into(),
    )));
    world.add_shape(back_wall.into());

    // smaller resolution, faster rendering
    let mut camera = Camera::new(200, 150, f64::consts::PI / 3.0);
    // let mut camera = Camera::new(2400 / 2, 1200 / 2, f64::consts::PI / 3.0);

    // larger resolution, slower rendering
    // let mut camera = Camera::new(2400, 1200, f64::consts::PI / 3.0);
    camera.set_transformation(Transformation::view(
        Tuple::point(0.0, 3.0, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    ));

    camera.render(&world).save("teapot")?;
    Ok(())
}
