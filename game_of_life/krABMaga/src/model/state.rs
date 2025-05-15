use std::any::Any;

use krabmaga::engine::fields::field_2d::Field2D;

use krabmaga::engine::location::Real2D;
use krabmaga::engine::{
    schedule::Schedule, state::State
};

use krabmaga::rand;
use krabmaga::rand::Rng;

use super::cell::Cell;

pub struct GameOfLifeState{
    pub step: u64,
    pub next_id: i32,
    pub dim: (f32, f32),
    pub cell_grid: Field2D<Cell>,
    pub initial_num_cells: i32,
    pub new_cells: Vec<Cell>,
    pub total_alive_cells: i32,
}

impl GameOfLifeState {
    pub fn new(dim: (f32, f32)) -> GameOfLifeState {
        GameOfLifeState {
            step: 0,
            next_id: 1,
            dim,
            cell_grid: Field2D::new(dim.0, dim.1, 1.0, true),
            initial_num_cells: 1,
            new_cells: Vec::new(),
            total_alive_cells: 0
        }
    }

    pub fn get_next_id(&mut self) -> i32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

impl State for GameOfLifeState {
    fn init(&mut self, schedule: &mut Schedule) {
        self.reset();

        generate_cells(self, schedule);
    }

    fn update(&mut self, step: u64) {
        self.step = step;
    }

    fn before_step(&mut self, _schedule: &mut Schedule) {
        self.new_cells.clear();
    }

    fn after_step(&mut self, schedule: &mut Schedule) {
        for cell in self.new_cells.iter() {
            self.cell_grid.set_object_location(*cell, cell.loc);
            schedule.schedule_repeating(Box::new(*cell), schedule.time + 1.0, 0);
        }
    }

    fn reset(&mut self) {
        self.step = 0;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }
}

fn generate_cells(state: &mut GameOfLifeState, schedule: &mut Schedule) {
    let mut rng = rand::thread_rng();

    for id in 0..state.initial_num_cells {
        let loc = Real2D {
            x: rng.gen_range(0..state.dim.0 as i32) as f32,
            y: rng.gen_range(0..state.dim.1 as i32) as f32,
        };
   
        let cell = Cell::new(
            id,
            loc
        );
        state.cell_grid.set_object_location(cell, loc);

        schedule.schedule_repeating(Box::new(cell), 0., 0);
    }
}
