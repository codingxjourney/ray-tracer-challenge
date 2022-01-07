extern crate ray_tracer_challenge as raytracer;
use raytracer::tuple::*; 
use num_traits::Float;

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
struct Environment<T> where T: Float{
    gravity: Tuple<T>,
    wind: Tuple<T>,
}

#[derive(Debug)]
struct Projectile<T> where T: Float {
    position: Tuple<T>,
    velocity: Tuple<T>,
}

impl<T> Projectile<T> where T: Float {
    fn new(position: Tuple<T>, velocity: Tuple<T>) -> Self {
        Projectile { position, velocity }
    }
}

impl<T> Environment<T> where T: Float {
    fn new(gravity: Tuple<T>, wind: Tuple<T>) -> Self {
        Environment{ gravity, wind }
    }
}

fn tick<T>(environment: &Environment<T>, projectile: &Projectile<T>) -> Projectile<T> where T: Float {
    Projectile::new(projectile.position + projectile.velocity, projectile.velocity + environment.gravity + environment.wind,)
}

