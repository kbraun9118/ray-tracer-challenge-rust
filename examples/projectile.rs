use ray_tracer_challenge::tuple::Tuple;

fn main() {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize() * 3.0,
    };
    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.0, 0.0, 0.0),
    };
    let mut t = 0;
    println!("Projectile starting at {:?}", p.position);
    while p.position.y > 0.0 {
        p = tick(&e, p);
        t += 1;
        println!("After {t} ticks projectile is at {:?}", p.position);
    }
    println!(
        "Projectile took {t} ticks to land, and traveled {}",
        p.position.x
    );
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}
