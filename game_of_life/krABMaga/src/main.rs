#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

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
    panic!("Visualization feature is not implemented yet. Please use the main function without visualization.");
}
