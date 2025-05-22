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

pub const DIMENSION: (f32, f32) = (10., 10.);
pub const INITIAL_STATE_PATH: &str = "../test_scenarios/10.txt";

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let step = 1000;

    let state = GameOfLifeState::new(DIMENSION);

    simulate!(state, step, 10, true);
}

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    let state = GameOfLifeState::new(DIMENSION);
    Visualization::default()
        .with_window_dimensions(800., 800.)
        .with_simulation_dimensions(DIMENSION.0, DIMENSION.1)
        .with_background_color(Color::WHITE)
        .with_name("Game of Life")
        .start::<VisState, GameOfLifeState>(VisState, state);
}
