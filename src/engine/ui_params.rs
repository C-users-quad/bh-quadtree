use crate::{
    engine::heatmap::HeatmapColor, physics::constants::DT_PHYSICS, utils::presets::Presets,
};

pub struct UIParams {
    pub open: bool,
    pub zoom_factor: f32,
    pub draw_particles: bool,
    pub draw_quadtree: bool,
    pub heatmap_color: HeatmapColor,
    pub selected_preset: Presets,
    pub sim_dt: f32,
}

impl UIParams {
    pub fn new(preset: Presets) -> Self {
        Self {
            open: true,
            zoom_factor: 1.0,
            draw_particles: true,
            draw_quadtree: true,
            heatmap_color: HeatmapColor::Ghost,
            selected_preset: preset,
            sim_dt: DT_PHYSICS,
        }
    }
}
