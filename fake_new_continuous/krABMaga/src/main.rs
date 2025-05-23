#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

use model::state::FakeNewsState;

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::vis_state::VisState, krabmaga::bevy::prelude::Color,
    krabmaga::visualization::visualization::Visualization,
};

mod model;

const DIMENSION: (f32, f32) = (100., 100.);

const POPULATION_SIZE: u32 = 1000;
const DISCRETIZATION: f32 = 10.0 / 1.5;
const TOROIDAL: bool = false;
const NEAREST_NEIGHBOURS_DISTANCE: f32 = 20.0;
const MOVE_NEIGHBOURS_DISTANCE: f32 = 10.0;
const MAX_MOVE_DISTANCE: f32 = 2.0;

pub const ALPHA: f32 = 0.02;
pub const BETA: f32 = 1.05;
pub const GAMMA: f32 = -0.95;
pub const DELTA: f32 = -0.96;
pub const THETA: f32 = -0.98;

pub const PARAM_A: f32 = 0.1;
pub const PARAM_B: f32 = 0.1;
pub const PARAM_C: f32 = 0.1;

pub const PROB_REMOVE_CONNECTION: f32 = 0.03;
pub const PROB_ADD_CONNECTION: f32 = 0.03;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let step: u64 = 50;

    let state = FakeNewsState::new(DIMENSION, POPULATION_SIZE, DISCRETIZATION, TOROIDAL);

    simulate!(state, step, 10);
}

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    let state = FakeNewsState::new(DIMENSION, POPULATION_SIZE, DISCRETIZATION, TOROIDAL);

    Visualization::default()
        .with_window_dimensions(1000., 700.)
        .with_simulation_dimensions(DIMENSION.0, DIMENSION.1)
        .with_background_color(Color::rgb(255., 255., 255.))
        .start::<VisState, FakeNewsState>(VisState, state);
}
