#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::vis_state::VisState,
    krabmaga::bevy::prelude::Color,
    krabmaga::visualization::visualization::Visualization,
};

// Global imports (needed for the simulation to run)
use crate::model::state::GameOfLifeState;

mod model;

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let step = 1000;

    let dim: (f32, f32) = (100., 100.);

    let state = GameOfLifeState::new(dim);

    simulate!(state, step, 10, true);
}

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    let dim: (f32, f32) = (100., 100.);

    let state = GameOfLifeState::new(dim);
    Visualization::default()
        .with_window_dimensions(800., 800.)
        .with_simulation_dimensions(dim.0, dim.1)
        .with_background_color(Color::WHITE)
        .with_name("Game of Life")
        .start::<VisState, GameOfLifeState>(VisState, state);
}
