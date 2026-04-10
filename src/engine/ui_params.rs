use crate::{engine::heatmap::HeatmapColor, utils::presets::Presets};

pub struct UIParams {
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
            zoom_factor: 1.0,
            draw_particles: true,
            draw_quadtree: false,
            heatmap_color: HeatmapColor::Nebula,
            selected_preset: Presets::Triple,
        }
    }
}
