use ray_tracer_challenge::{
    camera::Camera,
    color::{Color, Colors},
    error::RayTraceResult,
    point_light::PointLight,
    shape::{
        material::{
            pattern::{checker::CheckerPattern, stripes::StripePattern, Pattern},
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
    let mut camera = Camera::new(400, 200, 1.52);
    camera.set_transformation(Transformation::view(
        Tuple::point(-2.6, 1.5, -3.9),
        Tuple::point(-0.6, 1.0, -0.8),
        Tuple::vector(0.0, 1.0, 0.0),
    ));

    let mut world = World::new();

    let light = PointLight::new(Tuple::point(-4.9, 4.9, -1.0), Colors::White.into());
    world.add_light(light);

    let mut stripes =
        StripePattern::new(Color::new(0.45, 0.45, 0.45), Color::new(0.55, 0.55, 0.55));
    stripes.set_transformation(
        Transformation::identity()
            .scale(0.25, 0.25, 0.25)
            .rotate_y(1.5708),
    );

    let wall_material = Material::new()
        .with_pattern(stripes)
        .with_ambient(0.0)
        .with_diffuse(0.4)
        .with_specular(0.0)
        .with_reflective(0.3);

    let mut floor = Plane::new();
    floor.set_transformation(Transformation::identity().rotate_y(0.31415));
    floor.set_material(
        Material::new()
            .with_pattern(CheckerPattern::new(
                Color::new(0.35, 0.35, 0.35),
                Color::new(0.65, 0.65, 0.65),
            ))
            .with_specular(0.0)
            .with_reflective(0.4),
    );
    world.add_shape(floor.into());

    let mut ceiling = Plane::new();
    ceiling.set_transformation(Transformation::identity().translation(0.0, 5.0, 0.0));
    ceiling.set_material(
        Material::new()
            .with_color(Color::new(0.8, 0.8, 0.8))
            .with_ambient(0.3)
            .with_specular(0.0),
    );
    world.add_shape(ceiling.into());

    let mut west_wall = Plane::new();
    west_wall.set_transformation(
        Transformation::identity()
            .rotate_y(1.5708)
            .rotate_z(1.5708)
            .translation(-5.0, 0.0, 0.0),
    );
    west_wall.set_material(wall_material.clone());
    world.add_shape(west_wall.into());

    let mut east_wall = Plane::new();
    east_wall.set_transformation(
        Transformation::identity()
            .rotate_y(1.5708)
            .rotate_z(1.5708)
            .translation(5.0, 0.0, 0.0),
    );
    east_wall.set_material(wall_material.clone());
    world.add_shape(east_wall.into());

    let mut north_wall = Plane::new();
    north_wall.set_transformation(
        Transformation::identity()
            .rotate_x(1.5708)
            .translation(0.0, 0.0, 5.0),
    );
    north_wall.set_material(wall_material.clone());
    world.add_shape(north_wall.into());

    let mut south_wall = Plane::new();
    south_wall.set_transformation(
        Transformation::identity()
            .rotate_x(1.5708)
            .translation(0.0, 0.0, -5.0),
    );
    south_wall.set_material(wall_material.clone());
    world.add_shape(south_wall.into());

    //background spheres
    let mut sphere = Sphere::new();
    sphere.set_transformation(
        Transformation::identity()
            .scale(0.4, 0.4, 0.4)
            .translation(4.6, 0.4, 1.0),
    );
    sphere.set_material(
        Material::new()
            .with_color(Color::new(0.8, 0.5, 0.3))
            .with_shininess(50.0),
    );
    world.add_shape(sphere.into());

    let mut sphere = Sphere::new();
    sphere.set_transformation(
        Transformation::identity()
            .scale(0.3, 0.3, 0.3)
            .translation(4.7, 0.3, 0.4),
    );
    sphere.set_material(
        Material::new()
            .with_color(Color::new(0.9, 0.4, 0.5))
            .with_shininess(50.0),
    );
    world.add_shape(sphere.into());

    let mut sphere = Sphere::new();
    sphere.set_transformation(
        Transformation::identity()
            .scale(0.5, 0.5, 0.5)
            .translation(-1.0, 0.5, 4.5),
    );
    sphere.set_material(
        Material::new()
            .with_color(Color::new(0.4, 0.9, 0.6))
            .with_shininess(50.0),
    );
    world.add_shape(sphere.into());

    let mut sphere = Sphere::new();
    sphere.set_transformation(
        Transformation::identity()
            .scale(0.3, 0.3, 0.3)
            .translation(-1.7, 0.3, 4.7),
    );
    sphere.set_material(
        Material::new()
            .with_color(Color::new(0.4, 0.6, 0.9))
            .with_shininess(50.0),
    );
    world.add_shape(sphere.into());

    //forground spheres
    let mut sphere = Sphere::new();
    sphere.set_transformation(Transformation::identity().translation(-0.6, 1.0, 0.6));
    sphere.set_material(
        Material::new()
            .with_color(Color::new(1.0, 0.3, 0.2))
            .with_specular(0.4)
            .with_shininess(5.0),
    );
    world.add_shape(sphere.into());

    let mut sphere = Sphere::new();
    sphere.set_transformation(
        Transformation::identity()
            .scale(0.7, 0.7, 0.7)
            .translation(0.7, 0.7, -0.6),
    );
    sphere.set_material(
        Material::new()
            .with_color(Color::new(0.0, 0.0, 0.2))
            .with_ambient(0.0)
            .with_diffuse(0.4)
            .with_specular(0.9)
            .with_shininess(300.0)
            .with_reflective(0.9)
            .with_transparency(0.9)
            .with_refractive_index(1.5),
    );
    world.add_shape(sphere.into());

    let mut sphere = Sphere::new();
    sphere.set_transformation(
        Transformation::identity()
            .scale(0.5, 0.5, 0.5)
            .translation(-0.7, 0.5, -0.8),
    );
    sphere.set_material(
        Material::new()
            .with_color(Color::new(0.0, 0.2, 0.0))
            .with_ambient(0.0)
            .with_diffuse(0.4)
            .with_specular(0.9)
            .with_shininess(300.0)
            .with_reflective(0.9)
            .with_transparency(0.9)
            .with_refractive_index(1.5),
    );
    world.add_shape(sphere.into());

    camera.render(&world).save("reflect-refract")
}
