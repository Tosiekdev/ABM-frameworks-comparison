use krabmaga::engine::fields::{field::Field, field_2d::Field2D};
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::any::Any;
use krabmaga::*;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use std::cell::RefCell;

use crate::model::{
    cell::Cell,
    status::Status,
    education::Education,
    gender::Gender,
    age::Age
};

pub struct FakeNewsState {
    pub step: u64,
    pub num_cells: u32,
    pub field: Field2D<Cell>,
    pub discretization: f32,
    pub toroidal: bool,
    pub dim: (f32, f32),
}

impl FakeNewsState {
    pub fn new(dim: (f32, f32), num_cells: u32, d: f32, t: bool) -> FakeNewsState {
        FakeNewsState {
            step: 0,
            num_cells,
            discretization: d,
            field: Field2D::new(dim.0, dim.1, d, t),
            toroidal: t,
            dim,
        }
    }
}

impl State for FakeNewsState {
    fn reset(&mut self) {
        self.step = 0;
        self.field = Field2D::new(self.dim.0, self.dim.1, self.discretization, self.toroidal);
    }

    fn init(&mut self, schedule: &mut Schedule) {
        self.reset();
        generate_network(self, schedule);
        addplot!(
            String::from("Cells over time"),
            String::from("Steps"),
            String::from("Cells"),
            true
        );
    }

    fn update(&mut self, step: u64) {
        self.field.lazy_update();
        self.step = step;
    }

    #[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
    fn after_step(&mut self, _schedule: &mut Schedule) {
        let total_cells = RefCell::new(0);
        let total_exposed = RefCell::new(0);
        let total_infected = RefCell::new(0);
        let total_doubtful = RefCell::new(0);
        let total_recovered = RefCell::new(0);
        let total_susceptible = RefCell::new(0);

        self.field.iter_objects(|_loc, cell| {
            *total_cells.borrow_mut() += 1;
            match cell.status {
                Status::Exposed => *total_exposed.borrow_mut() += 1,
                Status::Infected => *total_infected.borrow_mut() += 1,
                Status::Doubtful => *total_doubtful.borrow_mut() += 1,
                Status::Recovered => *total_recovered.borrow_mut() += 1,
                Status::Susceptible => *total_susceptible.borrow_mut() += 1,
            }
        });

        plot!(
            String::from("Cells over time"),
            String::from("Cells"),
            self.step as f64,
            *total_cells.borrow_mut() as f64
        );

        plot!(
            String::from("Cells over time"),
            String::from("Susceptible"),
            self.step as f64,
            *total_susceptible.borrow_mut() as f64
        );

        plot!(
            String::from("Cells over time"),
            String::from("Exposed"),
            self.step as f64,
            *total_exposed.borrow_mut() as f64
        );

        plot!(
            String::from("Cells over time"),
            String::from("Infected"),
            self.step as f64,
            *total_infected.borrow_mut() as f64
        );

        plot!(
            String::from("Cells over time"),
            String::from("Doubtful"),
            self.step as f64,
            *total_doubtful.borrow_mut() as f64
        );

        plot!(
            String::from("Cells over time"),
            String::from("Recovered"),
            self.step as f64,
            *total_recovered.borrow_mut() as f64
        );
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }
}

fn generate_network(state: &mut FakeNewsState, schedule: &mut Schedule) {
    let mut cell_set = Vec::new();
    let mut rng = rand::thread_rng();

    for cell_id in 0..state.num_cells {
        let r1: f32 = rng.gen();
        let r2: f32 = rng.gen();

        let init_status: Status = rng.gen();
        let education: Education = rng.gen();
        let gender: Gender = rng.gen();
        let age: Age = rng.gen();
        
        let cell = Cell::new(
            cell_id,
            Real2D {
                x: state.dim.0 * r1,
                y: state.dim.1 * r2,
            },
            init_status,
            education,
            gender,
            age,
        );

        state.field.set_object_location(cell, cell.loc);
        schedule.schedule_repeating(Box::new(cell), 0.0, 0);
        cell_set.push(cell);
    }
}
