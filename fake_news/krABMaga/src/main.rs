#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

use model::state::FakeNewsState;


mod model;

const POPULATION_SIZE: u32 = 3_000;
const DISCRETIZATION: f32 = 10.0 / 1.5;
const TOROIDAL: bool = false;

pub const INIT_EDGES: usize = 2;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let step: u64 = 110;
    let dim: (f32, f32) = (100., 100.);

    let state = FakeNewsState::new(dim, POPULATION_SIZE, DISCRETIZATION, TOROIDAL);

    simulate!(state, step, 10);
}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    panic!(
        "Visualization feature is not yet implemented. Please use the non-visualization version."
    );
}
