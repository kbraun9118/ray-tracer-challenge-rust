use ray_tracer_challenge::{canvas::Canvas, color::Color, error::RayTraceResult, tuple::Tuple};

fn main() -> RayTraceResult<()> {
    let start = Tuple::point(0.0, 1.0, 0.0);
    let velocity = Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut p = Projectile {
        position: start,
        velocity,
    };

    let gravity = Tuple::vector(0.0, -0.1, 0.0);
    let wind = Tuple::vector(-0.01, 0.0, 0.0);
    let e = Environment { gravity, wind };

    let mut c = Canvas::new(900, 500);

    while p.position.y > 0.0 {
        plot(&mut c, &p);
        p = tick(&e, p);
    }

    c.save("simulation")?;
    Ok(())
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn plot(canvas: &mut Canvas, projectile: &Projectile) {
    let color = Color::new(1.0, 0.0, 0.0);
    let height = canvas.height();

    let x = projectile.position.x.round() as usize;
    let y = height - projectile.position.y.round() as usize;

    if x > canvas.width() || y > canvas.height() || x + y * canvas.width() > 450_000 {
        println!("how")
    }
    canvas[(x, y)] = color;
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}
