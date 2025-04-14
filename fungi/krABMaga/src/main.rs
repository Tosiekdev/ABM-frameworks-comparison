#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::vis_state::VisState,
    krabmaga::bevy::prelude::Color,
    krabmaga::bevy::prelude::FixedUpdate,
    krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D,
    krabmaga::visualization::fields::number_grid_2d::BatchRender,
    krabmaga::visualization::visualization::Visualization,
};

// Global imports (needed for the simulation to run)
use crate::model::state::FungiState;

mod model;

pub const DIRECT_OPPOSITE_CHANCE: f32 = 0.3;
pub const BROAD_OPPOSITE_CHANCE: f32 = 0.1;
pub const MAX_NEIGHBOR_BEFORE_STARVATION: f32 = 0.3;
pub const SAFE_NEIGHBOR: f32 = 0.0;

pub const HARVEST_AMOUNT: f32 = 1.;
pub const CONSUME_AMOUNT: f32 = 0.1;

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let step = 100;

    let dim: (i32, i32) = (400, 400);

    let state = FungiState::new(dim);

    simulate!(state, step, 10, true);
}

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    let dim: (i32, i32) = (100, 100);

    let state = FungiState::new(dim);
    let mut app = Visualization::default()
        .with_background_color(Color::rgb(255., 255., 255.))
        .with_simulation_dimensions(dim.0 as f32, dim.1 as f32)
        .with_window_dimensions(100., 100.)
        .setup::<VisState, FungiState>(VisState, state);
    app.add_systems(FixedUpdate, DenseNumberGrid2D::batch_render);
    app.run()
}
