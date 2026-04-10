use glium::implement_vertex;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    engine::heatmap::{HeatmapColor, compute_t},
    physics::{particle::Particle, quadnode::QuadNode},
};

// --- instanced circle rendering ---

/// one per particle, uploaded to per-instance buffer
#[derive(Copy, Clone)]
pub struct CircleInstance {
    i_center: [f32; 2],
    i_radius: f32,
    i_rgba: [f32; 4],
}
implement_vertex!(CircleInstance, i_center, i_radius, i_rgba);

impl CircleInstance {
    pub fn from_particles(particles: &[Particle]) -> Vec<CircleInstance> {
        particles
            .par_iter()
            .map(|p| CircleInstance {
                i_center: [p.pos.x, p.pos.y],
                i_radius: p.size,
                i_rgba: [1.0, 1.0, 1.0, 1.0],
            })
            .collect()
    }
}

/// one per node, uploaded to per-instance buffer
#[derive(Copy, Clone)]
pub struct QuadInstance {
    i_left: f32,
    i_top: f32,
    i_size: f32,
    i_rgba: [f32; 4],
}
implement_vertex!(QuadInstance, i_left, i_top, i_size, i_rgba);

impl QuadInstance {
    pub fn from_nodes(nodes: &[QuadNode], gradient: HeatmapColor) -> Vec<QuadInstance> {
        nodes
            .par_iter()
            .map(|n| {
                let bound = &n.boundary;
                let t = compute_t(bound, nodes);
                let color = HeatmapColor::get_color(gradient, t);
                QuadInstance {
                    i_left: bound.left,
                    i_top: bound.top,
                    i_size: bound.size,
                    i_rgba: color,
                }
            })
            .collect()
    }
}

/// a single unit quad [-1,1] in both axes, 6 verts, stays on GPU forever
#[derive(Copy, Clone)]
pub struct UnitQuadVertex {
    pub unit_pos: [f32; 2],
}
implement_vertex!(UnitQuadVertex, unit_pos);

pub fn unit_quad_vertices() -> [UnitQuadVertex; 6] {
    [
        UnitQuadVertex { unit_pos: [-1.0, -1.0] },
        UnitQuadVertex { unit_pos: [ 1.0, -1.0] },
        UnitQuadVertex { unit_pos: [-1.0,  1.0] },
        UnitQuadVertex { unit_pos: [ 1.0, -1.0] },
        UnitQuadVertex { unit_pos: [-1.0,  1.0] },
        UnitQuadVertex { unit_pos: [ 1.0,  1.0] },
    ]
}
