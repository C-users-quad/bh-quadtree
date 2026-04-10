use macroquad::{time::get_frame_time, window::next_frame};

use crate::{physics::particle::Particle, utils::presets::Presets, engine::renderer::Renderer, physics::simulation::Simulation};

mod physics;
mod engine;
mod utils;

#[macroquad::main("particles")]
async fn main() {
    let particles: Vec<Particle> = Presets::Triple.get_particles();
    let mut sim = Simulation::new(particles);
    let mut renderer = Renderer::new();

    loop {
        let dt = get_frame_time();
        sim.step();
        renderer.update(&mut sim, dt);
        renderer.draw(&mut sim);

        next_frame().await
    }
}
