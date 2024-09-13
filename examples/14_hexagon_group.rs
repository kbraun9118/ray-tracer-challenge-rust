use core::f64;

use ray_tracer_challenge::{
    camera::Camera, color::Colors, error::RayTraceResult, point_light::PointLight, shape::{
        cylinder::Cylinder,
        group::GroupContainer,
        sphere::Sphere,
        Shape,
    }, transformation::Transformation, tuple::Tuple, world::World
};

fn hexagon_corner() -> Sphere {
    let mut corner = Sphere::new();
    corner.set_transformation(
        Transformation::identity()
            .translation(0.0, 0.0, -1.0)
            .scale(0.25, 0.25, 0.25),
    );
    corner
}

fn hexagon_edge() -> Cylinder {
    let mut edge = Cylinder::new();
    edge.set_minimum(0.0);
    edge.set_maximum(1.0);
    edge.set_transformation(
        Transformation::identity()
            .translation(0.0, 0.0, -1.0)
            .rotate_y(-f64::consts::PI / 6.0)
            .rotate_z(-f64::consts::PI / 2.0)
            .scale(0.25, 1.0, 0.25),
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
    world.set_light(PointLight::new(
        Tuple::point(-10.0, 10.0, -10.0),
        Colors::White.into(),
    ));

    // smaller resolution, faster rendering
    let mut camera = Camera::new(200, 150, f64::consts::PI / 3.0);

    // larger resolution, slower rendering
    // let mut camera = Camera::new(1200, 600, PI / 3.0);
    camera.set_transformation(Transformation::view(
        Tuple::point(0.0, 1.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    ));

    camera.render(&world).save("hexagon")?;
    Ok(())
}
