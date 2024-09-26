use ray_tracer_challenge::{
    camera::Camera,
    color::Color,
    error::RayTraceResult,
    point_light::PointLight,
    shape::{
        material::{pattern::checker::CheckerPattern, Material},
        plane::Plane,
        sphere::Sphere,
        Shape,
    },
    transformation::Transformation,
    tuple::Tuple,
    world::World,
};

fn main() -> RayTraceResult<()> {
    let mut camera = Camera::new(800, 400, 1.0471966);
    camera.set_transformation(Transformation::view(
        Tuple::point(5.0, 1.5, -5.5),
        Tuple::point(0.0, 0.7, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    ));

    let mut world = World::new();
    let light = PointLight::new(Tuple::point(-5.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    world.add_light(light);

    let wall_material = Material::new()
        .with_pattern(CheckerPattern::new(
            Color::new(1.0, 1.0, 1.0),
            Color::new(0.5, 0.5, 0.5),
        ))
        .with_reflective(0.0);

    // let mut floor = Plane::new();
    // floor.set_material(wall_material.clone());
    // world.add_shape(floor.into());

    let mut left_wall = Plane::new();
    left_wall.set_material(wall_material.clone());
    left_wall.set_transformation(
        Transformation::identity()
            .rotate_z(1.570795)
            .translation(-15.0, 0.0, 0.0),
    );
    world.add_shape(left_wall.into());

    let mut right_wall = Plane::new();
    right_wall.set_material(wall_material.clone());
    right_wall.set_transformation(
        Transformation::identity()
            .rotate_x(1.570795)
            .translation(0.0, 0.0, 15.0),
    );
    world.add_shape(right_wall.into());

    let mut sphere = Sphere::new();
    sphere.set_material(
        Material::new()
            .with_color(Color::new(0.1, 0.1, 0.1))
            .with_transparency(1.0)
            .with_refractive_index(1.5),
    );
    sphere.set_transformation(Transformation::identity().translation(0.0, 1.5, 0.0));
    world.add_shape(sphere.into());

    let mut sphere = Sphere::new();
    sphere.set_material(
        Material::new()
            .with_color(Color::new(0.0, 0.0, 1.0))
            .with_diffuse(0.7)
            .with_specular(0.3)
            .with_reflective(0.2),
    );
    sphere.set_transformation(Transformation::identity().translation(-8.0, 1.0, 5.0));
    world.add_shape(sphere.into());

    let mut sphere = Sphere::new();
    sphere.set_material(
        Material::new()
            .with_color(Color::new(1.0, 0.0, 0.0))
            .with_diffuse(0.7)
            .with_specular(0.3)
            .with_reflective(0.2),
    );
    sphere.set_transformation(
        Transformation::identity()
            .scale(0.5, 0.5, 0.5)
            .translation(-1.0, 0.5, 5.0),
    );
    world.add_shape(sphere.into());

    let mut sphere = Sphere::new();
    sphere.set_material(
        Material::new()
            .with_color(Color::new(0.0, 1.0, 0.0))
            .with_diffuse(0.7)
            .with_specular(0.3)
            .with_reflective(0.2),
    );
    sphere.set_transformation(
        Transformation::identity()
            .scale(0.5, 0.5, 0.5)
            .translation(-2.3, 0.5, 0.77),
    );
    world.add_shape(sphere.into());

    camera.render(&world).save("refraction")
}
