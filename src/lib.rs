#[macro_use]
mod fuzzy_eq;

type F = f64;
pub const EPSILON: f64 = 0.00001;

pub mod tuple;
pub mod canvas;
pub mod matrix;
pub mod ray;
pub mod sphere;
pub mod body;
pub mod intersections;
pub mod light;
pub mod material;