use std::f32::consts::{PI, TAU};

use macroquad::{
    color::WHITE,
    text::draw_text,
    time::get_frame_time,
    window::next_frame,
};
use rand::random_range;

use crate::{
    constants::{CENTRAL_RADIUS, G, ORBITAL_RADIUS},
    particle::Particle,
    renderer::Renderer,
    simulation::Simulation,
    vec2::Vec2,
};

mod boundary;
mod constants;
mod particle;
mod quadnode;
mod quadtree;
mod renderer;
mod simulation;
mod vec2;

#[macroquad::main("main")]
async fn main() {
    let particles: Vec<Particle> = gen_triple();
    let mut sim = Simulation::new(particles);
    let mut renderer = Renderer::new();

    loop {
        let dt = get_frame_time();
        sim.step();
        renderer.update(&mut sim, dt);
        renderer.draw(&sim.particles, &sim.tree.nodes);

        next_frame().await
    }
}

// Spiral arms
pub fn gen_arms(n: usize, arms: usize, radius: f32, mass: f32) -> Vec<Particle> {
    let central_mass = mass * n as f32 * 10.0;
    let mut particles = vec![
        Particle::new(Vec2::zero(), Vec2::zero(), central_mass, CENTRAL_RADIUS)
    ];

    particles.extend((0..n).map(|i| {
        let arm = (i % arms) as f32;
        let t = fastrand::f32().max(0.01);
        let theta = arm * (TAU / arms as f32) + t * TAU * 1.5;
        let r = (t * radius + fastrand::f32() * radius * 0.15).max(1.0);
        let pos = Vec2::new(r * theta.cos(), r * theta.sin());

        let radial = pos / r;
        let tangent = Vec2::new(-radial.y, radial.x);
        let speed = (central_mass / r).sqrt();
        let vel = tangent * speed;

        Particle::new(pos, vel, mass, ORBITAL_RADIUS)
    }));

    particles
}

// Filaments: particles clustered along random line segments (cosmic web-ish)
pub fn gen_filaments(n: usize, num_filaments: usize, length: f32, mass: f32) -> Vec<Particle> {
    let per = n / num_filaments;
    (0..num_filaments).flat_map(|_| {
        let angle = fastrand::f32() * TAU;
        let cx = (fastrand::f32() - 0.5) * length;
        let cy = (fastrand::f32() - 0.5) * length;
        let dx = angle.cos();
        let dy = angle.sin();
        // perpendicular direction
        let px = -dy;
        let py = dx;
        // random wave params per filament
        let freq = 1.5 + fastrand::f32() * 2.0;   // how many waves along the filament
        let amp = length * (0.04 + fastrand::f32() * 0.08); // wave height, subtle
        (0..per).map(move |_| {
            let t = (fastrand::f32() - 0.5) * length;
            let wave = (t / length * freq * TAU).sin() * amp;
            let spread = (fastrand::f32() - 0.5) * length * 0.02;
            let pos = Vec2::new(
                cx + dx * t + px * (wave + spread),
                cy + dy * t + py * (wave + spread),
            );
            Particle::new(pos, Vec2::zero(), mass, ORBITAL_RADIUS)
        })
    }).collect()
}

// Multiple gaussian blobs at random positions
pub fn gen_blobs(n: usize, num_blobs: usize, spread: f32, mass: f32) -> Vec<Particle> {
    let per = n / num_blobs;
    let centers: Vec<Vec2> = (0..num_blobs).map(|_| {
        Vec2::new((fastrand::f32() - 0.5) * spread, (fastrand::f32() - 0.5) * spread)
    }).collect();
    (0..num_blobs).flat_map(|b| {
        let c = centers[b];
        (0..per).map(move |_| {
            let r = fastrand::f32() * spread * 0.15;
            let theta = fastrand::f32() * TAU;
            Particle::new(c + Vec2::new(r * theta.cos(), r * theta.sin()), Vec2::zero(), mass, ORBITAL_RADIUS)
        })
    }).collect()
}

/// Generates a galaxy that forms a spiral
fn gen_spiral(particles: &mut Vec<Particle>) {
    particles.extend(gen_galaxy(
        Vec2::zero(),
        2500.0,
        100_000,
        100_000_000.0,
        100.0,
        Vec2::zero(),
        Vec2::new(100.0, -100.0),
    ))
}

/// Two galaxies on a collision course
fn gen_collision(particles: &mut Vec<Particle>) {
    particles.extend(gen_galaxy(
        Vec2::new(-1500.0, 0.0), 1200.0, 40_000,
        100_000_000.0, 100.0,
        Vec2::new(30.0, 0.0),   // moving right
        Vec2::new(-1500.0, 0.0),
    ));
    particles.extend(gen_galaxy(
        Vec2::new(1500.0, 0.0), 1200.0, 40_000,
        100_000_000.0, 100.0,
        Vec2::new(-30.0, 0.0),  // moving left
        Vec2::new(1500.0, 0.0),
    ));
}

/// Two galaxies doing a glancing pass — produces tidal arms
fn gen_glancing(particles: &mut Vec<Particle>) {
    particles.extend(gen_galaxy(
        Vec2::new(-2000.0, -400.0), 1000.0, 40_000,
        100_000_000.0, 100.0,
        Vec2::new(25.0, 5.0),
        Vec2::new(-2000.0, -400.0),
    ));
    particles.extend(gen_galaxy(
        Vec2::new(2000.0, 400.0), 1000.0, 40_000,
        100_000_000.0, 100.0,
        Vec2::new(-25.0, -5.0),
        Vec2::new(2000.0, 400.0),
    ));
}

/// Ring galaxy — particles on a thin annulus
fn gen_ring(center: Vec2, inner_r: f32, outer_r: f32, n: i32, mass: f32) -> Vec<Particle> {
    let mut result = Vec::new();
    result.push(Particle::new(center, Vec2::zero(), 100_000_000.0, CENTRAL_RADIUS));
    (0..n).for_each(|_| {
        let u = random_range(0f32..1f32);
        let r = inner_r + (outer_r - inner_r) * u;
        let theta = random_range(0f32..2f32 * PI);
        let pos = Vec2::new(r * theta.cos() + center.x, r * theta.sin() + center.y);
        let offset = pos - center;
        let v_mag = (G * 100_000_000.0 / r).sqrt();
        let vel = Vec2::new(-offset.y, offset.x) / offset.mag() * v_mag;
        result.push(Particle::new(pos, vel, mass, ORBITAL_RADIUS));
    });
    result
}

/// Uniform sphere collapse — particles in a 2D disk with no initial velocity, just fall inward
fn gen_collapse(center: Vec2, radius: f32, n: i32, mass: f32) -> Vec<Particle> {
    (0..n).map(|_| {
        let u = random_range(0f32..1f32);
        let r = radius * u.sqrt();
        let theta = random_range(0f32..2f32 * PI);
        let pos = Vec2::new(r * theta.cos() + center.x, r * theta.sin() + center.y);
        Particle::new(pos, Vec2::zero(), mass, ORBITAL_RADIUS)
    }).collect()
}

/// Three galaxies in a slow waltz
fn gen_triple() -> Vec<Particle> {
    let mut result: Vec<Particle> = Vec::new();
    let angles = [0.0f32, 2.0 * PI / 3.0, 4.0 * PI / 3.0];
    let orbit_r = 2000.0;
    let v = 15.0;
    for a in angles {
        let center = Vec2::new(orbit_r * a.cos(), orbit_r * a.sin());
        let vel = Vec2::new(-a.sin() * v, a.cos() * v);
        result.extend(gen_galaxy(
            center, 800.0, 25_000,
            80_000_000.0, 100.0,
            vel, center,
        ));
    }
    result
}

/// Creates a `Galaxy` represented as a collection of `num_particles`
/// orbital particles with mass `orbital_mass` that orbit the point `center`.
///
/// The uniform disk that the galaxy is composed of has a radius `radius`
/// and a velocity `galaxy_vel`.
///
/// The galaxy contains a central particle with position `central_mass_pos` and mass
/// `central_mass`. This emulates the black hole at the center of a real galaxy.
///
/// # Returns
///  - a `Vec<Particle>` that contains every particle in the galaxy, all with the
///    properly initialized positions, velocities, masses and radii.
fn gen_galaxy(
    center: Vec2,
    radius: f32,
    num_particles: i32,
    central_mass: f32,
    orbital_mass: f32,
    galaxy_vel: Vec2,
    central_mass_pos: Vec2,
) -> Vec<Particle> {
    // initialize the galaxy and push the central particle
    let mut galaxy: Vec<Particle> = Vec::new();
    let central_particle =
        Particle::new(central_mass_pos, Vec2::zero(), central_mass, CENTRAL_RADIUS);
    galaxy.push(central_particle);

    // create and push the orbital particles
    (0..num_particles).for_each(|_| {
        let particle_pos = get_uniform_disk_pos(center, radius, CENTRAL_RADIUS);
        galaxy.push(Particle::new(
            particle_pos,
            get_tangential_velocity(
                particle_pos,
                center,
                central_mass,
                num_particles,
                orbital_mass,
                radius,
            ),
            orbital_mass,
            ORBITAL_RADIUS,
        ));
    });

    // applies the galaxies initial velocity to every particle
    galaxy.iter_mut().for_each(|p| {
        p.vel += galaxy_vel;
    });

    galaxy
}

/// Provides a random position inside of the uniform disk with radius `disk_radius`
/// and center `center`.
///
/// `r` is sampled as `disk_radius² * sqrt(u)` rather than uniformly,
/// to correct for the fact that area scales with `r` — without this, particles would
/// cluster toward the center.
///
/// # Returns
///  - A [`Vec2`] position vector containing the randomly sampled position.
fn get_uniform_disk_pos(center: Vec2, disk_radius: f32, inner_radius: f32) -> Vec2 {
    let u = random_range(0f32..1f32);
    let r = (disk_radius * u.sqrt()).max(inner_radius);
    let theta = random_range(0f32..2f32 * PI);
    Vec2::new(r * theta.cos() + center.x, r * theta.sin() + center.y)
}

/// Determines the initial velocity a [`Particle`] should have in a uniform disk
/// in order to remain in a circular orbit around the center.
///
/// # Returns
///  - A [`Vec2`] with the tangential velocity of a particle at initial position `pos`.
fn get_tangential_velocity(
    pos: Vec2,
    center: Vec2,
    central_mass: f32,
    num_particles: i32,
    orbital_mass: f32,
    disk_radius: f32,
) -> Vec2 {
    // get the total mass enclosed by the particle
    let offset = pos - center;
    let r = offset.mag();
    let total_mass_orbital = num_particles as f32 * orbital_mass;
    let total_area_disk = PI * disk_radius.powi(2);
    let disk_mass_density = total_mass_orbital / total_area_disk;
    let area_enclosed = PI * r * r;
    let halo_mass = central_mass * (r / disk_radius); // mass grows linearly with r
    let mass_enclosed = disk_mass_density * area_enclosed + central_mass + halo_mass;

    // construct the tangential velocity vector
    let v_t_mag = (G * mass_enclosed / r).sqrt();
    let mut v_t_dir = Vec2::new(-offset.y, offset.x);
    v_t_dir /= v_t_dir.mag();
    v_t_dir * v_t_mag
}
