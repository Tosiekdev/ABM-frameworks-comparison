use core::fmt;
use std::hash::{Hash, Hasher};

use krabmaga::engine::fields::field_2d::Location2D;
use krabmaga::engine::location::Real2D;
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
    pub loc: Real2D,
    pub state: LifeState,
}

impl Cell {
    pub fn new(
        id: u32,
        loc: Real2D,
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

    fn next_cell_state(&mut self, state: &GameOfLifeState) {
        let live_neighbors = state
            .cell_grid
            .get_neighbors_within_relax_distance(self.loc, 1.0)
            .iter()
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
        state.cell_grid.set_object_location(*self, self.loc);
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

impl Location2D<Real2D> for Cell {
    fn get_location(self) -> Real2D {
        self.loc
    }

    fn set_location(&mut self, loc: Real2D) {
        self.loc = loc;
    }
}
