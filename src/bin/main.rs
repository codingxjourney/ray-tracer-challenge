extern crate ray_tracer_challenge as raytracer;
use raytracer::tuple::*; 

fn main() {
    let environment = Environment::new(Tuple::vector(0.0, -0.1, 0.0), Tuple::vector(0.0001, 0.0, 0.0));
    let projectile = Projectile::new(Tuple::vector(0.0, 1.0, 0.0), Tuple::vector(0.02, 0.0, 0.0));

    println!("Environment: {:?}", environment);

    let mut current = projectile;
    let mut iteration = 0;
    while current.position.y > 0.0 {
        println!("{}: {:?}", iteration, current);
        current = tick(&environment, &current);
        iteration += 1;
    }
    println!("Finished -> {}: {:?}", iteration, current);
}

#[derive(Debug)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    fn new(position: Tuple, velocity: Tuple) -> Self {
        Projectile { position, velocity }
    }
}

impl Environment {
    fn new(gravity: Tuple, wind: Tuple) -> Self {
        Environment{ gravity, wind }
    }
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    Projectile::new(projectile.position + projectile.velocity, projectile.velocity + environment.gravity + environment.wind,)
}

