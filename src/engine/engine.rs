use egui_glium::{
    EguiGlium,
    egui_winit::egui::{self, Color32, FontId, RichText, Slider, ViewportId},
};
use glium::{
    Frame, Program, Surface, VertexBuffer,
    backend::glutin::{Display, SimpleWindowBuilder},
    glutin::surface::WindowSurface,
    index::{NoIndices, PrimitiveType::TrianglesList},
    uniform,
    winit::{dpi::PhysicalSize, event_loop::EventLoop, keyboard::KeyCode, window::Window},
};

use crate::{
    engine::{
        camera::Camera,
        heatmap::HeatmapColor,
        keyboard::Keyboard,
        mouse::Mouse,
        ui_params::UIParams,
        vertex::{CircleInstance, QuadInstance, UnitQuadVertex, unit_quad_vertices},
    },
    physics::{
        constants::NUM_PARTICLES, particle::Particle, quadnode::QuadNode, simulation::Simulation,
    },
    utils::presets::Presets,
};

pub struct Engine {
    display: Display<WindowSurface>,
    // permanent unit quad, never changes
    unit_quad: VertexBuffer<UnitQuadVertex>,
    // per-instance buffers, rewritten each frame
    circle_instance_cache: Vec<CircleInstance>,
    quad_instance_cache: Vec<QuadInstance>,
    circle_instances: VertexBuffer<CircleInstance>,
    quad_instances: VertexBuffer<QuadInstance>,
    c_program: Program,
    sq_program: Program,
    indices: NoIndices,
    dt: f32,
    pub ui_params: UIParams,
    pub window: Window,
    pub keyboard: Keyboard,
    pub mouse: Mouse,
    pub camera: Camera,
    pub egui_glium: EguiGlium,
}

impl Engine {
    pub fn new(preset: Presets) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::builder().build().unwrap();
        let (window, display) = SimpleWindowBuilder::new()
            .with_title("N-Body Sim")
            .build(&event_loop);

        let indices = NoIndices(TrianglesList);

        let c_vert_src = include_str!("../../shaders/circle_vert.glsl");
        let c_frag_src = include_str!("../../shaders/circle_frag.glsl");
        let c_program = Program::from_source(&display, c_vert_src, c_frag_src, None).unwrap();

        let sq_vert_src = include_str!("../../shaders/square_vert.glsl");
        let sq_frag_src = include_str!("../../shaders/square_frag.glsl");
        let sq_program = Program::from_source(&display, sq_vert_src, sq_frag_src, None).unwrap();

        let unit_quad = VertexBuffer::immutable(&display, &unit_quad_vertices()).unwrap();
        let circle_instances = VertexBuffer::dynamic(&display, &[]).unwrap();
        let quad_instances = VertexBuffer::dynamic(&display, &[]).unwrap();
        let circle_instance_cache = Vec::with_capacity(NUM_PARTICLES as usize);
        let quad_instance_cache = Vec::with_capacity(NUM_PARTICLES as usize * 4);

        let keyboard = Keyboard::new();
        let mouse = Mouse::new();
        let camera = Camera::new();
        let egui_glium = EguiGlium::new(ViewportId::ROOT, &display, &window, &event_loop);
        let ui_params = UIParams::new(preset);

        (
            Self {
                display,
                window,
                unit_quad,
                circle_instance_cache,
                quad_instance_cache,
                circle_instances,
                quad_instances,
                c_program,
                sq_program,
                indices,
                keyboard,
                mouse,
                camera,
                egui_glium,
                ui_params,
                dt: 0.01,
            },
            event_loop,
        )
    }

    pub fn resize_window(&self, new_size: PhysicalSize<u32>) {
        self.display.resize(new_size.into());
    }

    fn handle_keyboard_input(&mut self, sim: &mut Simulation) {
        if self.keyboard.just_released(KeyCode::Space) {
            sim.paused = !sim.paused;
        }
        if self.keyboard.just_released(KeyCode::Escape) {
            self.ui_params.open = !self.ui_params.open;
        }
        if self.keyboard.just_released(KeyCode::KeyQ) {
            self.ui_params.draw_quadtree = !self.ui_params.draw_quadtree;
        }
        if self.keyboard.just_released(KeyCode::KeyE) {
            self.ui_params.draw_particles = !self.ui_params.draw_particles;
        }
        if self.keyboard.just_released(KeyCode::KeyR) {
            sim.load_preset(self.ui_params.selected_preset);
        }
    }

    pub fn update(&mut self, sim: &mut Simulation, dt: f32) {
        self.dt = dt;
        let (w, h) = self.display.get_framebuffer_dimensions();
        self.camera.pan(&self.mouse, h as f32);
        self.camera.update(&self.mouse, (w as f32, h as f32));
        self.handle_keyboard_input(sim);
        self.keyboard.end_frame();
        self.mouse.end_frame();
    }

    fn write_instance_buffers(&mut self, particles: &[Particle], nodes: &[QuadNode]) {
        if self.ui_params.draw_particles {
            self.circle_instance_cache.clear();
            CircleInstance::from_particles(particles, &mut self.circle_instance_cache);
            if self.circle_instance_cache.len() != self.circle_instances.len() {
                self.circle_instances =
                    VertexBuffer::dynamic(&self.display, &self.circle_instance_cache).unwrap();
            } else if !self.circle_instance_cache.is_empty() {
                self.circle_instances.write(&self.circle_instance_cache);
            }
        }

        if self.ui_params.draw_quadtree {
            self.quad_instance_cache.clear();
            QuadInstance::from_nodes(
                nodes,
                self.ui_params.heatmap_color,
                &mut self.quad_instance_cache,
            );
            if self.quad_instance_cache.len() != self.quad_instances.len() {
                self.quad_instances =
                    VertexBuffer::dynamic(&self.display, &self.quad_instance_cache).unwrap();
            } else if !self.quad_instance_cache.is_empty() {
                self.quad_instances.write(&self.quad_instance_cache);
            }
        }
    }

    pub fn draw(&mut self, sim: &mut Simulation) {
        let (w, h) = self.display.get_framebuffer_dimensions();
        let inverse_aspect = h as f32 / w as f32;
        let uniforms = uniform! {
            cam_pos: [self.camera.pos.x, self.camera.pos.y],
            cam_zoom: self.camera.zoom,
            inverse_aspect: inverse_aspect,
            screen_size: [w as f32, h as f32],
        };

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        self.write_instance_buffers(&sim.particles, &sim.tree.nodes);

        if self.ui_params.draw_quadtree && !(self.quad_instances.len() == 0) {
            let per_instance = self.quad_instances.per_instance().unwrap();
            target
                .draw(
                    (&self.unit_quad, per_instance),
                    &self.indices,
                    &self.sq_program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
        }

        if self.ui_params.draw_particles && !(self.circle_instances.len() == 0) {
            let per_instance = self.circle_instances.per_instance().unwrap();
            target
                .draw(
                    (&self.unit_quad, per_instance),
                    &self.indices,
                    &self.c_program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
        }

        self.draw_ui(&mut target, sim);
        target.finish().unwrap();
    }

    fn draw_ui(&mut self, target: &mut Frame, sim: &mut Simulation) {
        if !self.ui_params.open {
            return;
        }
        self.egui_glium.run(&self.window, |ctx| {
            egui::Window::new("Controls").show(ctx, |ui| {
                ui.label(
                    RichText::new("Rendering")
                        .font(FontId::proportional(20.0))
                        .color(Color32::WHITE),
                );
                ui.checkbox(&mut self.ui_params.draw_particles, "Draw Particles");
                ui.checkbox(&mut self.ui_params.draw_quadtree, "Draw Heatmap");
                egui::ComboBox::from_label("Heatmap Gradient")
                    .selected_text(self.ui_params.heatmap_color.label())
                    .show_ui(ui, |ui| {
                        for gradient in HeatmapColor::ALL {
                            ui.selectable_value(
                                &mut self.ui_params.heatmap_color,
                                *gradient,
                                gradient.label(),
                            );
                        }
                    });
                ui.separator();
                ui.label(
                    RichText::new("Simulation")
                        .font(FontId::proportional(20.0))
                        .color(Color32::WHITE),
                );
                let pause_text = if sim.paused { "Unpause" } else { "Pause" };
                if ui.button(pause_text).clicked() {
                    sim.paused = !sim.paused;
                }
                egui::ComboBox::from_label("Sim Presets")
                    .selected_text(self.ui_params.selected_preset.label())
                    .show_ui(ui, |ui| {
                        for preset in Presets::ALL {
                            ui.selectable_value(
                                &mut self.ui_params.selected_preset,
                                *preset,
                                preset.label(),
                            );
                        }
                    });
                if ui.button("Load Preset").clicked() {
                    sim.load_preset(self.ui_params.selected_preset);
                }
                ui.add(Slider::new(&mut self.ui_params.sim_dt, 0.0..=0.1).text("Speed"));
                ui.separator();
                ui.label(
                    RichText::new("Info")
                        .font(FontId::proportional(20.0))
                        .color(Color32::WHITE),
                );
                ui.label(format!("FPS: {}", self.dt.recip() as u32));
                ui.label(format!("Particles: {}", sim.particles.len()));
                ui.label(format!("Nodes: {}", sim.tree.nodes.len()));
                ctx.set_zoom_factor(self.ui_params.zoom_factor);
            });
        });
        self.egui_glium.paint(&self.display, target);
    }
}
