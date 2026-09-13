mod engine;
mod physics;
mod utils;

use std::time::Instant;

use glium::winit::{self, event::MouseScrollDelta};

use crate::{engine::engine::Engine, physics::simulation::Simulation, utils::presets::Presets};

fn main() {
    let preset = Presets::Collapse;
    let mut sim = Simulation::new(preset);
    let (mut engine, event_loop) = Engine::new(preset);
    let mut last_frame = Instant::now();

    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| {
            match event {
                winit::event::Event::WindowEvent { event, .. } => {
                    let event_response = engine.egui_glium.on_event(&engine.window, &event);
                    if event_response.consumed {
                        return;
                    }
                    match event {
                        winit::event::WindowEvent::CloseRequested => {
                            // close window.
                            window_target.exit();
                        }
                        winit::event::WindowEvent::Resized(new_size) => {
                            // resize window.
                            engine.resize_window(new_size);
                        }
                        winit::event::WindowEvent::KeyboardInput { event, .. } => {
                            // update keyboard state
                            engine.keyboard.handle_key_input(event);
                        }
                        winit::event::WindowEvent::MouseInput { state, button, .. } => {
                            // update mouse button state
                            engine.mouse.handle_button_input(state, button);
                        }
                        winit::event::WindowEvent::CursorMoved { position, .. } => {
                            // update mouse position
                            engine.mouse.handle_cursor_moved(position);
                        }
                        winit::event::WindowEvent::RedrawRequested => {
                            // draw the game
                            engine.draw(&mut sim);
                        }
                        winit::event::WindowEvent::MouseWheel { delta, .. } => {
                            // update camera if mouse is scrolling
                            let y = match delta {
                                MouseScrollDelta::LineDelta(_, y) => y,
                                MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                            };
                            engine.camera.handle_scroll(y, &engine.keyboard);
                        }
                        _ => {}
                    }
                }
                winit::event::Event::AboutToWait => {
                    // game loop code.
                    // get dt
                    let now = Instant::now();
                    let dt = now.duration_since(last_frame).as_secs_f32();
                    last_frame = now;

                    // do updates
                    sim.step(engine.ui_params.sim_dt);
                    engine.update(&mut sim, dt);

                    // request draw
                    engine.window.request_redraw();
                }
                _ => {}
            }
        })
        .unwrap();
}
