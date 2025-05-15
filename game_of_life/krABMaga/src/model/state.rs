use std::any::Any;

use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::field_2d::Field2D;

use krabmaga::engine::location::Real2D;
use krabmaga::engine::{
    schedule::Schedule, state::State
};
use krabmaga::rand;
use krabmaga::rand::Rng;

use super::cell::{Cell, LifeState};

pub struct GameOfLifeState{
    pub step: u64,
    pub next_id: u32,
    pub dim: (f32, f32),
    pub cell_grid: Field2D<Cell>,
    pub new_cells: Vec<Cell>,
}

impl GameOfLifeState {
    pub fn new(dim: (f32, f32)) -> GameOfLifeState {
        GameOfLifeState {
            step: 0,
            next_id: 1,
            dim,
            cell_grid: Field2D::new(dim.0, dim.1, 1.0, true),
            new_cells: Vec::new(),
        }
    }

    pub fn get_next_id(&mut self) -> u32 {
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

        self.cell_grid.lazy_update();
    }

    fn reset(&mut self) {
        self.step = 0;
        self.cell_grid = Field2D::new(self.dim.0, self.dim.1, 1.0, true);
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

    for x in 0..state.dim.0 as i32 {
        for y in 0..state.dim.1 as i32 {
            let loc = Real2D {
                x: x as f32,
                y: y as f32,
            };
    
            let is_alive = rng.gen_bool(0.5);

            let cell = Cell::new(
                state.get_next_id(),
                loc,
                if is_alive {
                    LifeState::Alive
                } else {
                    LifeState::Dead
                },
            );
            state.cell_grid.set_object_location(cell, loc);
    
            schedule.schedule_repeating(Box::new(cell), 0., 0);
        }
    }
}
