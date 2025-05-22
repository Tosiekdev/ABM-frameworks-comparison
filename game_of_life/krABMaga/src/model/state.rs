use std::any::Any;
use std::fs::File;
use std::io::{BufRead, BufReader};

use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::field::Field;

use krabmaga::engine::location::Int2D;
use krabmaga::engine::{
    schedule::Schedule, state::State
};

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use std::time::Instant;

use super::cell::{Cell, LifeState};

pub struct GameOfLifeState{
    pub step: u64,
    pub next_id: u32,
    pub dim: (i32, i32),
    pub cell_grid: DenseGrid2D<Cell>,
    pub initial_state_path: String,
    pub read: usize,

    
    #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
    start_time: Option<Instant>,
    #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
    end_step: u64,
}

impl GameOfLifeState {
    #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
    pub fn new(dim: (i32, i32), initial_state_path: &str, end_step: u64) -> GameOfLifeState {
        GameOfLifeState {
            step: 0,
            next_id: 1,
            dim,
            cell_grid: DenseGrid2D::new(dim.0, dim.1),
            initial_state_path: initial_state_path.to_string(),
            read: 0,
            start_time: None,
            end_step,
        }
    }

    #[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
    pub fn new(dim: (i32, i32), initial_state_path: &str) -> GameOfLifeState {
        GameOfLifeState {
            step: 0,
            next_id: 1,
            dim,
            cell_grid: DenseGrid2D::new(dim.0, dim.1),
            initial_state_path: initial_state_path.to_string(),
            read: 0,
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

        #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
        if self.step == 0 {
            self.start_time = Some(Instant::now());
        }

        self.cell_grid.lazy_update();
        self.read = 1 - self.read;

        #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
        if self.step == self.end_step {
            if let Some(start_time) = self.start_time {
                println!(
                    "Took {} ms",
                    Instant::now()
                        .duration_since(start_time)
                        .as_millis()
                );
            }
        }
    }

    fn reset(&mut self) {
        self.step = 0;
        self.read = 0;
        self.cell_grid = DenseGrid2D::new(self.dim.0, self.dim.1);
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
    let file = match File::open(&state.initial_state_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open initial state file: {}", e);
            return;
        }
    };
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut y = 0;

    loop {
        match reader.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                for (x, ch) in line.trim_end().chars().enumerate() {
                    let loc = Int2D {
                        x: x as i32,
                        y: state.dim.1 - 1 - y as i32,
                    };

                    let is_alive = match ch {
                        '1' => true,
                        '0' => false,
                        _ => {
                            eprintln!("Invalid character in initial state file: {}", ch);
                            continue;
                        }
                    };

                    let cell = Cell::new(
                        state.get_next_id(),
                        loc,
                        if is_alive {
                            LifeState::Alive
                        } else {
                            LifeState::Dead
                        },
                    );
                    state.cell_grid.set_object_location(cell, &loc);
                    schedule.schedule_repeating(Box::new(cell), 0., 0);
                }

                y += 1;
                line.clear();
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }
}
