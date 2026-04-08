
use macroquad::{time::get_frame_time, window::next_frame};

use crate::{
    particle::Particle,
    presets::Presets,
    renderer::Renderer,
    simulation::Simulation,
};

mod boundary;
mod constants;
mod particle;
mod presets;
mod quadnode;
mod quadtree;
mod renderer;
mod simulation;
mod vec2;

#[macroquad::main("main")]
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
