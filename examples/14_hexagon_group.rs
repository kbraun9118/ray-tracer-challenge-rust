use core::f64;

use ray_tracer_challenge::{
    camera::Camera,
    color::Colors,
    error::RayTraceResult,
    point_light::PointLight,
    shape::{
        cylinder::Cylinder,
        group::GroupContainer,
        material::{pattern::checker::CheckerPattern, Material},
        plane::Plane,
        sphere::Sphere,
        Shape,
    },
    transformation::Transformation,
    tuple::Tuple,
    world::World,
};

fn hexagon_corner() -> Sphere {
    let mut corner = Sphere::new();
    corner.set_material(Material::new().with_transparency(0.25));
    corner.set_transformation(
        Transformation::identity()
            .scale(0.25, 0.25, 0.25)
            .translation(0.0, 0.0, -1.0),
    );
    corner
}

fn hexagon_edge() -> Cylinder {
    let mut edge = Cylinder::new();
    edge.set_minimum(0.0);
    edge.set_maximum(1.0);
    edge.set_material(Material::new().with_transparency(0.25));
    edge.set_transformation(
        Transformation::identity()
            .scale(0.25, 1.0, 0.25)
            .rotate_z(-f64::consts::PI / 2.0)
            .rotate_y(-f64::consts::PI / 6.0)
            .translation(0.0, 0.0, -1.0),
    );

    edge
}

fn hexagon_side() -> GroupContainer {
    let side = GroupContainer::default();
    side.add_child(hexagon_corner().into());
    side.add_child(hexagon_edge().into());

    side
}

fn hexagon() -> GroupContainer {
    let hex = GroupContainer::default();

    for n in 0..5 {
        let side = hexagon_side();
        side.borrow_mut().set_transformation(
            Transformation::identity().rotate_y((n as f64) * f64::consts::PI / 3.0),
        );
        hex.add_child(side.into());
    }

    hex
}

fn main() -> RayTraceResult<()> {
    let mut world = World::new();
    world.add_shape(hexagon().into());
    let mut back = Plane::new();
    back.set_transformation(
        Transformation::identity()
            .rotate_x(f64::consts::PI / 2.0)
            .translation(0.0, 0.0, 5.0),
    );
    back.set_material(Material::default().with_pattern(CheckerPattern::new(
        Colors::Black.into(),
        Colors::Red.into(),
    )));
    world.add_shape(back.into());
    world.set_light(PointLight::new(
        Tuple::point(-10.0, 10.0, -10.0),
        Colors::White.into(),
    ));

    // smaller resolution, faster rendering
    // let mut camera = Camera::new(200, 150, f64::consts::PI / 3.0);
    let mut camera = Camera::new(2400 / 2, 1200 / 2, f64::consts::PI / 3.0);

    // larger resolution, slower rendering
    // let mut camera = Camera::new(2400, 1200, f64::consts::PI / 3.0);
    camera.set_transformation(Transformation::view(
        Tuple::point(0.0, 1.5, -5.0),
        Tuple::point(0.0, 0.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    ));

    camera.render(&world).save("hexagon")?;
    Ok(())
}
