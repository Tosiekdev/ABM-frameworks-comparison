#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

// Global imports (needed for the simulation to run)
use crate::model::fungi::FungiState;

mod model;

pub const DIRECT_OPPOSITE_CHANCE: f32 = 0.3;
pub const BROAD_OPPOSITE_CHANCE: f32 = 0.1;
pub const MAX_NEIGHBOR_BEFORE_STARVATION: f32 = 0.3;
pub const SAFE_NEIGHBOR: f32 = 0.0;

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let step = 100;

    let dim: (i32, i32) = (400, 400);

    let state = FungiState::new(dim);

    simulate!(state, step, 1, true);
}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    panic!(
        "The visualization feature is not yet implemented. Please use the simulation feature instead."
    );
}
