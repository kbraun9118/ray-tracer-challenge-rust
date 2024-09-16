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

fn hexagon_corner(material: Material) -> Sphere {
    let mut corner = Sphere::new();
    corner.set_transformation(
        Transformation::identity()
            .scale(0.25, 0.25, 0.25)
            .translation(0.0, 0.0, -1.0),
    );
    corner.set_material(material);
    corner
}

fn hexagon_edge(material: Material) -> Cylinder {
    let mut edge = Cylinder::new();
    edge.set_minimum(0.0);
    edge.set_maximum(1.0);
    edge.set_transformation(
        Transformation::identity()
            .scale(0.25, 1.0, 0.25)
            .rotate_z(-f64::consts::PI / 2.0)
            .rotate_y(-f64::consts::PI / 6.0)
            .translation(0.0, 0.0, -1.0),
    );
    edge.set_material(material);

    edge
}

fn hexagon_side(material: Material) -> GroupContainer {
    let side = GroupContainer::default();
    side.add_child(hexagon_corner(material.clone()).into());
    side.add_child(hexagon_edge(material.clone()).into());

    side
}

fn hexagon(material: Material) -> GroupContainer {
    let hex = GroupContainer::default();

    for n in 0..=5 {
        let side = hexagon_side(material.clone());
        side.borrow_mut().set_transformation(
            Transformation::identity()
                .rotate_y((n as f64) * f64::consts::PI / 3.0)
                .translation(0.0, 0.5, 0.0),
        );
        hex.add_child(side.into());
    }

    hex
}

fn main() -> RayTraceResult<()> {
    let mut world = World::new();
    world.add_shape(
        hexagon(
            Material::default()
                .with_transparency(1.0)
                .with_reflective(1.0)
                .with_refractive_index(1.52),
        )
        .into(),
    );

    // let mut sphere = Sphere::new();
    // sphere.set_material(
    //     Material::default()
    //         .with_transparency(1.0)
    //         .with_reflective(1.0)
    //         .with_refractive_index(1.52),
    // );
    // world.add_shape(sphere.into());

    world.set_light(PointLight::new(
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

    // larger resolution, slower rendering
    // let mut camera = Camera::new(1200, 600, PI / 3.0);
    camera.set_transformation(Transformation::view(
        Tuple::point(0.0, 3.0, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    ));

    camera.render(&world).save("hexagon")?;
    Ok(())
}
