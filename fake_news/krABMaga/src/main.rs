#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

use model::state::FakeNewsState;

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::vis_state::VisState, krabmaga::bevy::prelude::Color,
    krabmaga::bevy::prelude::FixedUpdate, krabmaga::visualization::fields::network::NetworkRender,
    krabmaga::visualization::visualization::Visualization,
};

mod model;

const POPULATION_SIZE: u32 = 50;
const DISCRETIZATION: f32 = 10.0 / 1.5;
const TOROIDAL: bool = false;

pub const INIT_EDGES: usize = 2;

pub const ALPHA: f32 = 0.1;
pub const BETA: f32 = 0.1;
pub const GAMMA: f32 = 0.1;
pub const DELTA: f32 = 0.1;
pub const THETA: f32 = 0.1;

pub const PARAM_A: f32 = 0.1;
pub const PARAM_B: f32 = 0.1;
pub const PARAM_C: f32 = 0.1;

pub const PROB_REMOVE_CONNECTION: f32 = 0.03;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let step: u64 = 110;
    let dim: (f32, f32) = (100., 100.);

    let state = FakeNewsState::new(dim, POPULATION_SIZE, DISCRETIZATION, TOROIDAL);

    simulate!(state, step, 10);
}

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    let dim: (f32, f32) = (500., 500.);
    let state = FakeNewsState::new(dim, POPULATION_SIZE, DISCRETIZATION, TOROIDAL);

    let mut app = Visualization::default()
        .with_window_dimensions(1000., 700.)
        .with_simulation_dimensions(dim.0, dim.1)
        .with_background_color(Color::rgb(255., 255., 255.))
        .setup::<VisState, FakeNewsState>(VisState, state);
    app.add_systems(FixedUpdate, FakeNewsState::render);
    app.run();
}
