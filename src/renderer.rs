use egui_macroquad::egui::{self, Color32, FontId, RichText};
use macroquad::{
    color::{BLACK, Color, WHITE},
    input::{KeyCode, is_key_down, is_key_released, mouse_wheel},
    shapes::draw_rectangle,
    time::{draw_fps, get_fps},
    window::{clear_background, screen_height, screen_width},
};

use crate::{
    constants::MIN_NODE_SIZE, particle::Particle, presets::Presets, quadnode::QuadNode,
    quadtree::QuadTree, simulation::Simulation, vec2::Vec2,
};

pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,
    target_zoom: f32,
    speed: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Vec2::zero(),
            zoom: 0.1,
            target_zoom: 0.1,
            speed: 1000.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.update_speed();
        self.update_pos(dt);
        self.update_zoom();
    }

    /// converts a world position to a screen drawing position
    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let center = Vec2::new(screen_width() * 0.5, screen_height() * 0.5);
        (world_pos - self.pos) * self.zoom + center
    }

    fn update_zoom(&mut self) {
        if !is_key_down(KeyCode::LeftControl) {
            let mouse_y = mouse_wheel().1;
            if mouse_y > 0.0 {
                self.target_zoom *= 1.1;
                self.target_zoom = self.target_zoom.min(50.0);
            } else if mouse_y < 0.0 {
                self.target_zoom /= 1.1;
                self.target_zoom = self.target_zoom.max(0.001);
            }
        }
        // smoothly interpolate toward target zoom
        self.zoom += (self.target_zoom - self.zoom) * 0.15;
    }

    fn update_pos(&mut self, dt: f32) {
        let move_speed = self.speed / self.zoom;
        if is_key_down(KeyCode::W) {
            self.pos.y -= move_speed * dt;
        }
        if is_key_down(KeyCode::A) {
            self.pos.x -= move_speed * dt;
        }
        if is_key_down(KeyCode::S) {
            self.pos.y += move_speed * dt;
        }
        if is_key_down(KeyCode::D) {
            self.pos.x += move_speed * dt;
        }
    }

    fn update_speed(&mut self) {
        if is_key_down(KeyCode::LeftControl) {
            let mouse_y = mouse_wheel().1;
            if mouse_y > 0.0 {
                self.speed += 50.0;
            } else if mouse_y < 0.0 {
                self.speed -= 50.0;
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum HeatmapColor {
    Solar,
    Aurora,
    Ocean,
    Lava,
    Toxic,
    Steel,
    Cold,
    Inferno,
    Matrix,
    Nebula,
    Diverge,
    Rainbow,
    Ghost,
}

impl HeatmapColor {
    fn label(&self) -> &'static str {
        match self {
            Self::Solar => "Solar",
            Self::Aurora => "Aurora",
            Self::Ocean => "Ocean",
            Self::Lava => "Lava",
            Self::Toxic => "Toxic",
            Self::Steel => "Steel",
            Self::Cold => "Cold",
            Self::Inferno => "Inferno",
            Self::Matrix => "Matrix",
            Self::Nebula => "Nebula",
            Self::Diverge => "Diverge",
            Self::Rainbow => "Rainbow",
            Self::Ghost => "Ghost",
        }
    }

    /// returns the color for the quadtree heatmap
    pub fn get_color(color: HeatmapColor, t: f32) -> Color {
        match color {
            HeatmapColor::Solar => color_solar(t),
            HeatmapColor::Aurora => color_aurora(t),
            HeatmapColor::Ocean => color_ocean(t),
            HeatmapColor::Lava => color_lava(t),
            HeatmapColor::Toxic => color_toxic(t),
            HeatmapColor::Steel => color_steel(t),
            HeatmapColor::Cold => color_cold(t),
            HeatmapColor::Inferno => color_inferno(t),
            HeatmapColor::Matrix => color_matrix(t),
            HeatmapColor::Nebula => color_nebula(t),
            HeatmapColor::Diverge => color_diverge(t),
            HeatmapColor::Rainbow => color_rainbow(t),
            HeatmapColor::Ghost => color_ghost(t),
        }
    }

    const ALL: &'static [Self] = &[
        Self::Solar,
        Self::Aurora,
        Self::Ocean,
        Self::Lava,
        Self::Toxic,
        Self::Steel,
        Self::Cold,
        Self::Inferno,
        Self::Matrix,
        Self::Nebula,
        Self::Diverge,
        Self::Rainbow,
        Self::Ghost,
    ];
}

struct UIParams {
    pub open: bool,
    pub zoom_factor: f32,
    pub draw_particles: bool,
    pub draw_quadtree: bool,
    pub heatmap_color: HeatmapColor,
    pub selected_preset: Presets,
}

impl UIParams {
    pub fn new() -> Self {
        Self {
            open: true,
            zoom_factor: 1.5,
            draw_particles: true,
            draw_quadtree: false,
            heatmap_color: HeatmapColor::Nebula,
            selected_preset: Presets::Triple,
        }
    }
}

pub struct Renderer {
    pub cam: Camera,
    ui_params: UIParams,
}

impl Renderer {
    const MIN_PARTICLE_RADIUS: f32 = 1.0;

    pub fn new() -> Self {
        Self {
            cam: Camera::new(),
            ui_params: UIParams::new(),
        }
    }

    pub fn update(&mut self, sim: &mut Simulation, dt: f32) {
        self.cam.update(dt);
        self.input(sim);
    }

    fn input(&mut self, sim: &mut Simulation) {
        if is_key_released(KeyCode::Space) {
            sim.paused = !sim.paused;
        }
        if is_key_released(KeyCode::Escape) {
            self.ui_params.open = !self.ui_params.open;
        }
        if is_key_released(KeyCode::Q) {
            self.ui_params.draw_quadtree = !self.ui_params.draw_quadtree;
        }
        if is_key_released(KeyCode::E) {
            self.ui_params.draw_particles = !self.ui_params.draw_particles;
        }
    }

    pub fn draw(&mut self, sim: &mut Simulation) {
        clear_background(BLACK);
        self.draw_quadtree(&sim.tree.nodes);
        self.draw_particles(&sim.particles);
        self.draw_ui(sim);
    }

    fn draw_ui(&mut self, sim: &mut Simulation) {
        if !self.ui_params.open {
            return;
        }

        egui_macroquad::ui(|ctx| {
            egui::Window::new("Options").show(ctx, |ui| {
                ui.label(
                    RichText::new("Rendering")
                        .font(FontId::proportional(20.0))
                        .color(Color32::WHITE),
                );
                ui.checkbox(&mut self.ui_params.draw_particles, "Draw Particles");
                ui.checkbox(&mut self.ui_params.draw_quadtree, "Draw Heatmap");
                egui::ComboBox::from_label("Heatmap Gradient")
                    .selected_text(self.ui_params.heatmap_color.label()) // what shows in the box
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
                let pause_button_text = if sim.paused { "Unpause" } else { "Pause" };
                if ui.button(pause_button_text).clicked() {
                    sim.paused = !sim.paused;
                }
                egui::ComboBox::from_label("Sim Presets")
                    .selected_text(self.ui_params.selected_preset.label()) // what shows in the box
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
                ui.separator();
                ui.label(
                    RichText::new("Info")
                        .font(FontId::proportional(20.0))
                        .color(Color32::WHITE),
                );
                ui.label(format!("FPS: {}", get_fps()));
                ui.label(format!("Particle Count: {}", sim.particles.len()));
                ctx.set_zoom_factor(self.ui_params.zoom_factor);
            });
        });

        egui_macroquad::draw();
    }

    fn draw_particles(&self, particles: &[Particle]) {
        if !self.ui_params.draw_particles {
            return;
        }

        let w = screen_width();
        let h = screen_height();
        for p in particles {
            let dp = self.cam.world_to_screen(p.pos);
            // radius minimum prevents particles from flickering at low zoom
            let r = (p.size * self.cam.zoom).max(Self::MIN_PARTICLE_RADIUS);
            if dp.x + r < 0.0 || dp.x - r > w || dp.y + r < 0.0 || dp.y - r > h {
                continue;
            }
            draw_rectangle(dp.x, dp.y, r, r, WHITE);
        }
    }

    fn draw_quadtree(&self, nodes: &[QuadNode]) {
        if !self.ui_params.draw_quadtree {
            return;
        }
        let w = screen_width();
        let h = screen_height();

        for n in nodes {
            let bound = &n.boundary;
            let dp = self.cam.world_to_screen(Vec2::new(bound.left, bound.top));
            let ds = bound.size * self.cam.zoom;
            if dp.x + ds <= 0.0 || dp.x > w || dp.y + ds <= 0.0 || dp.y > h {
                continue;
            }
            let root_size = nodes[QuadTree::ROOT].boundary.size;
            let depth = (root_size / n.boundary.size).log2();
            let max_depth = (root_size / MIN_NODE_SIZE).log2();
            let t = (depth / max_depth as f32).clamp(0.0, 1.0);
            let color = HeatmapColor::get_color(self.ui_params.heatmap_color, t);
            draw_rectangle(dp.x, dp.y, ds, ds, color);
        }
    }
}

// GRADIENTS, for quadtree drawing.

// Black → Deep Red → Gold → White (solar flare)
fn color_solar(t: f32) -> Color {
    let (r, g, b) = if t < 0.33 {
        let s = t / 0.33;
        (s, 0.0, 0.0)
    } else if t < 0.66 {
        let s = (t - 0.33) / 0.33;
        (1.0, s * 0.6, 0.0)
    } else {
        let s = (t - 0.66) / 0.34;
        (1.0, 0.6 + s * 0.4, s)
    };
    Color::new(r, g, b, 1.0)
}

// Black → Teal → Lime → White (aurora)
fn color_aurora(t: f32) -> Color {
    let (r, g, b) = if t < 0.33 {
        let s = t / 0.33;
        (0.0, s * 0.8, s * 0.6)
    } else if t < 0.66 {
        let s = (t - 0.33) / 0.33;
        (s * 0.2, 0.8 + s * 0.2, 0.6 - s * 0.6)
    } else {
        let s = (t - 0.66) / 0.34;
        (0.2 + s * 0.8, 1.0, s)
    };
    Color::new(r, g, b, 1.0)
}

// Black → Navy → Electric Blue → Cyan → White (deep ocean)
fn color_ocean(t: f32) -> Color {
    let (r, g, b) = if t < 0.25 {
        let s = t / 0.25;
        (0.0, 0.0, s * 0.5)
    } else if t < 0.5 {
        let s = (t - 0.25) / 0.25;
        (0.0, s * 0.3, 0.5 + s * 0.5)
    } else if t < 0.75 {
        let s = (t - 0.5) / 0.25;
        (0.0, 0.3 + s * 0.7, 1.0)
    } else {
        let s = (t - 0.75) / 0.25;
        (s, 1.0, 1.0)
    };
    Color::new(r, g, b, 1.0)
}

// Black → Crimson → Orange → Yellow (lava)
fn color_lava(t: f32) -> Color {
    let (r, g, b) = if t < 0.4 {
        let s = t / 0.4;
        (s * 0.8, 0.0, 0.0)
    } else if t < 0.7 {
        let s = (t - 0.4) / 0.3;
        (0.8 + s * 0.2, s * 0.4, 0.0)
    } else {
        let s = (t - 0.7) / 0.3;
        (1.0, 0.4 + s * 0.6, s * 0.3)
    };
    Color::new(r, g, b, 1.0)
}

// Black → Dark Green → Bright Green → White (toxic)
fn color_toxic(t: f32) -> Color {
    let (r, g, b) = if t < 0.33 {
        let s = t / 0.33;
        (0.0, s * 0.4, 0.0)
    } else if t < 0.66 {
        let s = (t - 0.33) / 0.33;
        (s * 0.3, 0.4 + s * 0.6, 0.0)
    } else {
        let s = (t - 0.66) / 0.34;
        (0.3 + s * 0.7, 1.0, s)
    };
    Color::new(r, g, b, 1.0)
}

// Greyscale with a blue tint at mid-depth
fn color_steel(t: f32) -> Color {
    let base = t;
    let blue_bump = (-(t - 0.4).powi(2) / 0.05).exp() * 0.4;
    Color::new(base, base, (base + blue_bump).clamp(0.0, 1.0), 1.0)
}

// Black → Blue → White
fn color_cold(t: f32) -> Color {
    let r = (t * 2.0 - 1.0).clamp(0.0, 1.0);
    let g = (t * 2.0 - 1.0).clamp(0.0, 1.0);
    let b = (t * 2.0).clamp(0.0, 1.0);
    Color::new(r, g, b, 1.0)
}

// Black → Red → Yellow → White (inferno-ish)
fn color_inferno(t: f32) -> Color {
    let r = (t * 2.0).clamp(0.0, 1.0);
    let g = (t * 2.0 - 1.0).clamp(0.0, 1.0);
    let b = (t * 4.0 * (1.0 - t)).clamp(0.0, 1.0); // hump in the middle
    Color::new(r, g, b, 1.0)
}

// Black → Green → Cyan → White
fn color_matrix(t: f32) -> Color {
    let r = (t * 2.0 - 1.0).clamp(0.0, 1.0);
    let g = (t * 2.0).clamp(0.0, 1.0);
    let b = (t * 2.0 - 1.0).clamp(0.0, 1.0);
    Color::new(r, g, b, 1.0)
}

// Black → Deep Blue → Purple → Pink → White
fn color_nebula(t: f32) -> Color {
    let (r, g, b) = if t < 0.33 {
        let s = t / 0.33;
        (0.0, 0.0, s) // black → deep blue
    } else if t < 0.66 {
        let s = (t - 0.33) / 0.33;
        (s * 0.8, 0.0, 1.0) // deep blue → purple
    } else {
        let s = (t - 0.66) / 0.34;
        (0.8 + s * 0.2, s, 1.0) // purple → pink → white
    };
    Color::new(r, g, b, 1.0)
}

// Cyan → Black → Orange (diverging, black at mid-depth)
fn color_diverge(t: f32) -> Color {
    if t < 0.5 {
        let s = t * 2.0;
        Color::new(0.0, s * 0.8, s, 1.0) // black → cyan
    } else {
        let s = (t - 0.5) * 2.0;
        Color::new(s, s * 0.4, 0.0, 1.0) // black → orange
    }
}

fn color_rainbow(t: f32) -> Color {
    let h = t * 360.0; // red → orange → yellow → green → blue → violet
    let (r, g, b) = hsv_to_rgb(h, 1.0, 1.0);
    Color::new(r, g, b, 1.0)
}

// Ghostly: transparent black → glowing teal (good with low alpha on fills)
fn color_ghost(t: f32) -> Color {
    let r = 0.0;
    let g = (t * 1.5 - 0.2).clamp(0.0, 1.0);
    let b = t;
    let a = (t * 3.0).clamp(0.0, 1.0); // sparse nodes nearly invisible
    Color::new(r, g, b, a)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let h = h % 360.0;
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    match (h / 60.0) as u32 {
        0 => (m + c, m + x, m),
        1 => (m + x, m + c, m),
        2 => (m, m + c, m + x),
        3 => (m, m + x, m + c),
        4 => (m + x, m, m + c),
        _ => (m + c, m, m + x),
    }
}
