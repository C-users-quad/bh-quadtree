use crate::{constants::NUM_PARTICLES, particle::Particle, simulation::Simulation};
use rand::Rng;

pub mod boundary;
pub mod constants;
pub mod particle;
pub mod quadnode;
pub mod quadtree;
pub mod simulation;
pub mod vec_2;

pub fn main() {
    let u = rand::random_range(0.0..1.0);
    println!("{u}");
}
