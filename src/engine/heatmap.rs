use crate::{
    physics::{constants::MIN_NODE_SIZE, quadnode::QuadNode, quadtree::QuadTree},
    utils::boundary::Boundary,
};

#[derive(PartialEq, Clone, Copy)]
pub enum HeatmapColor {
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
    pub fn label(&self) -> &'static str {
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
    pub fn get_color(color: HeatmapColor, t: f32) -> [f32; 4] {
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

    pub const ALL: &'static [Self] = &[
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

// GRADIENTS, for quadtree drawing.

#[inline]
pub fn compute_t(boundary: &Boundary, nodes: &[QuadNode]) -> f32 {
    let root_size = nodes[QuadTree::ROOT].boundary.size;
    let depth = (root_size / boundary.size).log2();
    let max_depth = (root_size / MIN_NODE_SIZE).log2();
    (depth / max_depth as f32).clamp(0.0, 1.0)
}

// Black → Deep Red → Gold → White (solar flare)
fn color_solar(t: f32) -> [f32; 4] {
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
    [r, g, b, 1.0]
}

// Black → Teal → Lime → White (aurora)
fn color_aurora(t: f32) -> [f32; 4] {
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
    [r, g, b, 1.0]
}

// Black → Navy → Electric Blue → Cyan → White (deep ocean)
fn color_ocean(t: f32) -> [f32; 4] {
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
    [r, g, b, 1.0]
}

// Black → Crimson → Orange → Yellow (lava)
fn color_lava(t: f32) -> [f32; 4] {
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
    [r, g, b, 1.0]
}

// Black → Dark Green → Bright Green → White (toxic)
fn color_toxic(t: f32) -> [f32; 4] {
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
    [r, g, b, 1.0]
}

// Greyscale with a blue tint at mid-depth
fn color_steel(t: f32) -> [f32; 4] {
    let base = t;
    let blue_bump = (-(t - 0.4).powi(2) / 0.05).exp() * 0.4;
    let r = base;
    let g = base;
    let b = (base + blue_bump).clamp(0.0, 1.0);
    [r, g, b, 1.0]
}

// Black → Blue → White
fn color_cold(t: f32) -> [f32; 4] {
    let r = (t * 2.0 - 1.0).clamp(0.0, 1.0);
    let g = (t * 2.0 - 1.0).clamp(0.0, 1.0);
    let b = (t * 2.0).clamp(0.0, 1.0);
    [r, g, b, 1.0]
}

// Black → Red → Yellow → White (inferno-ish)
fn color_inferno(t: f32) -> [f32; 4] {
    let r = (t * 2.0).clamp(0.0, 1.0);
    let g = (t * 2.0 - 1.0).clamp(0.0, 1.0);
    let b = (t * 4.0 * (1.0 - t)).clamp(0.0, 1.0); // hump in the middle
    [r, g, b, 1.0]
}

// Black → Green → Cyan → White
fn color_matrix(t: f32) -> [f32; 4] {
    let r = (t * 2.0 - 1.0).clamp(0.0, 1.0);
    let g = (t * 2.0).clamp(0.0, 1.0);
    let b = (t * 2.0 - 1.0).clamp(0.0, 1.0);
    [r, g, b, 1.0]
}

// Black → Deep Blue → Purple → Pink → White
fn color_nebula(t: f32) -> [f32; 4] {
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
    [r, g, b, 1.0]
}

// Cyan → Black → Orange (diverging, black at mid-depth)
fn color_diverge(t: f32) -> [f32; 4] {
    if t < 0.5 {
        let s = t * 2.0;
        [0.0, s * 0.8, s, 1.0] // black -> cyan
    } else {
        let s = (t - 0.5) * 2.0;
        [s, s * 0.4, 0.0, 1.0] // black -> orange
    }
}

fn color_rainbow(t: f32) -> [f32; 4] {
    let h = t * 360.0; // red → orange → yellow → green → blue → violet
    let (r, g, b) = hsv_to_rgb(h, 1.0, 1.0);
    [r, g, b, 1.0]
}

// Ghostly: transparent black → glowing teal (good with low alpha on fills)
fn color_ghost(t: f32) -> [f32; 4] {
    let r = 0.0;
    let g = (t * 1.5 - 0.2).clamp(0.0, 1.0);
    let b = t;
    let a = (t * 3.0).clamp(0.0, 1.0); // sparse nodes nearly invisible
    [r, g, b, a]
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
