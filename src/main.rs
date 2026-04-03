use std::f32::consts::PI;

use ::rand::random_range;
use macroquad::prelude::*;

use crate::{
    constants::{
        CENTRAL_MASS, CENTRAL_RADIUS, G, MAX_DISK_RADIUS_2, MIN_DISK_RADIUS_2, NUM_PARTICLES,
        ORBITAL_MASS, ORBITAL_RADIUS,
    },
    particle::Particle,
    simulation::Simulation,
    vec_2::Vec2,
};

pub mod boundary;
pub mod constants;
pub mod particle;
pub mod quadnode;
pub mod quadtree;
pub mod simulation;
pub mod vec_2;

#[macroquad::main("bh-quadtree")]
async fn main() {
    let particles = make_particles();
    let mut sim = Simulation::new(particles);
    let mut cam: (f32, f32) = (0.0, 0.0);
    let mut cam_speed: f32 = 1.0;
    let mut cam_zoom: f32 = 1.0;
    loop {
        clear_background(BLACK);
        let start = std::time::Instant::now();
        sim.step();
        let elapsed = start.elapsed();
        draw_text(
            &format!("Sim step: {:?}", elapsed),
            30f32,
            20f32,
            40f32,
            WHITE,
        );
        if is_key_down(KeyCode::F) {
            if is_key_down(KeyCode::LeftShift) {
                cam_speed = (cam_speed - 10.0f32).max(1.0f32);
            } else {
                cam_speed = (cam_speed + 10.0f32).min(100.0f32);
            }
        }
        if is_key_down(KeyCode::Z) {
            if is_key_down(KeyCode::LeftShift) {
                cam_zoom = (cam_zoom + 0.025f32).min(10.0f32);
            } else {
                cam_zoom = (cam_zoom - 0.025f32).max(0.1f32);
            }
        }
        if is_key_down(KeyCode::W) {
            cam.1 += 1.0 * cam_speed;
        }
        if is_key_down(KeyCode::A) {
            cam.0 += 1.0 * cam_speed;
        }
        if is_key_down(KeyCode::S) {
            cam.1 -= 1.0 * cam_speed;
        }
        if is_key_down(KeyCode::D) {
            cam.0 -= 1.0 * cam_speed;
        }
        sim.particles.iter().for_each(|p: &Particle| {
            draw_circle(
                p.position.x * cam_zoom + cam.0,
                p.position.y * cam_zoom + cam.1,
                (p.radius * cam_zoom).max(1.0f32),
                WHITE,
            );
        });
        next_frame().await;
    }
}

/// Creates a vector of [`Particle`]s with length [`NUM_PARTICLES`] + 1, where the
/// \"+1\" represents the central particle, located at index 0.
///
/// All the particles are initialized with positions in a uniform disk centered around
/// the origin, which is where the central particle is located.
///
/// This initializes a beautiful example of the Barnes-Hut [`quadtree::QuadTree`]
/// in action, with N particles orbiting a massive central particle.
///
/// # Returns
///  - A `Vec` containing NUM_PARTICLES + 1 particles.
fn make_particles() -> Vec<Particle> {
    let mut particles: Vec<Particle> = Vec::with_capacity(NUM_PARTICLES + 1);

    let central_particle = Particle::new(
        Vec2::default(),
        Vec2::default(),
        CENTRAL_MASS,
        CENTRAL_RADIUS,
    );
    particles.push(central_particle);

    (0..NUM_PARTICLES).for_each(|_| {
        let particle_pos = get_uniform_disk_pos();
        particles.push(Particle::new(
            particle_pos,
            get_tangential_velocity(&particle_pos),
            ORBITAL_MASS,
            ORBITAL_RADIUS,
        ));
    });

    particles
}

/// Provides a random position inside of the uniform disk with outer radius
/// sqrt([`MAX_DISK_RADIUS_2`]) and inner radius sqrt([`MIN_DISK_RADIUS_2`]).
///
/// `r` is sampled as `sqrt(u * (r_max² - r_min²) + r_min²)` rather than uniformly,
/// to correct for the fact that area scales as r² — without this, particles would
/// cluster toward the center.
///
/// # Returns
///  - A [`Vec2`] position vector containing the randomly sampled position.
fn get_uniform_disk_pos() -> Vec2 {
    let u = random_range(0f32..1f32);
    let r = ((MAX_DISK_RADIUS_2 - MIN_DISK_RADIUS_2) * u + MIN_DISK_RADIUS_2).sqrt();
    let theta = random_range(0f32..2f32 * PI);
    Vec2::new(r * theta.cos(), r * theta.sin())
}

/// Determines the initial velocity a [`Particle`] should have in a uniform disk
/// with a central particle with mass [`CENTRAL_MASS`] in order to remain
/// in a circular orbit around the central particle.
///
/// # Returns
///  - A [`Vec2`] with the tangential velocity of a particle at initial position `pos`.
fn get_tangential_velocity(pos: &Vec2) -> Vec2 {
    let r = pos.len();
    let v_t_mag = (G * CENTRAL_MASS / r).sqrt();
    let mut v_t_dir = Vec2::new(-pos.y, pos.x);
    v_t_dir /= v_t_dir.len();
    v_t_dir * v_t_mag
}
