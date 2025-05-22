use core::fmt;
use std::hash::{Hash, Hasher};

// use krabmaga::engine::fields::field_2d::Location2D;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::{
    agent::Agent, state::State
};

use super::state::GameOfLifeState;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LifeState {
    Alive,
    Dead,
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub id: u32,
    pub loc: Int2D,
    pub state: LifeState,
}

impl Cell {
    pub fn new(
        id: u32,
        loc: Int2D,
        state: LifeState,
    ) -> Cell {
        Cell {
            id,
            loc,
            state,
        }
    }

    #[allow(dead_code)]
    pub fn as_agent(self) -> Box<dyn Agent> {
        Box::new(self)
    }

    fn get_toroidal_neighbors<'a>(self: &'a Self, state: &'a GameOfLifeState) -> impl Iterator<Item = Cell> + 'a {
        const DIRECTIONS: [(i32, i32); 8] = [
            (-1, -1), (0, -1), (-1, 0),
            (-1, 1),           (0, 1),
            (1, 1),   (1, 0),  (1, -1),
        ];

        let dim_x: i32 = state.dim.0;
        let dim_y: i32 = state.dim.1;
        let x: i32 = self.loc.x;
        let y: i32 = self.loc.y;

        #[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
        let rlocs = state.cell_grid.locs[state.read].borrow();

        DIRECTIONS.into_iter().filter_map(move |(dx, dy)| {
            let nx: i32 = (x + dx + dim_x) % dim_x;
            let ny: i32 = (y + dy + dim_y) % dim_y;

            #[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))] {
                let index = (nx * dim_y + ny) as usize;
                let cell_list = &rlocs[index];
                if cell_list.is_empty() {
                    None
                } else {
                    Some(cell_list[0])
                }
            }
            
            #[cfg(any(feature = "visualization", feature = "visualization_wasm"))] {
                let loc: Int2D = Int2D { x: nx, y: ny };
                let vec: &Vec<Cell> = state.cell_grid.loc2objs.get_read(&loc)?;
                return if vec.is_empty() { None } else { Some(vec[0]) };
            }
        })
    }
    
    fn next_cell_state(&mut self, state: &GameOfLifeState) {
        let live_neighbors = self
            .get_toroidal_neighbors(state)
            .filter(|cell| cell.state == LifeState::Alive && cell.id != self.id)
            .count();

        let next_state = match (self.state, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (LifeState::Alive, x) if x < 2 => LifeState::Dead,

                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (LifeState::Alive, 2) | (LifeState::Alive, 3) => LifeState::Alive,

                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (LifeState::Alive, x) if x > 3 => LifeState::Dead,

                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (LifeState::Dead, 3) => LifeState::Alive,

                    // All other cells remain in the same state.
                    (_otherwise, _) => self.state,
                };

        // Update the cell's state
        self.state = next_state;
    }
}

impl Agent for Cell {
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any_mut().downcast_mut::<GameOfLifeState>().unwrap();

        self.next_cell_state(state);
        state.cell_grid.set_object_location(*self, &self.loc);
    }

    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        false
    }
}

impl Eq for Cell {}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.id == other.id
    }
}

impl Hash for Cell {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
