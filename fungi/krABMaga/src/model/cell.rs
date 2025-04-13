use core::fmt;
use std::hash::{Hash, Hasher};

use krabmaga::engine::{agent::Agent, location::Int2D, state::State};
use krabmaga::rand;
use krabmaga::rand::Rng;

use super::state::FungiState;
use crate::{
    DIRECT_OPPOSITE_CHANCE, BROAD_OPPOSITE_CHANCE,
    MAX_NEIGHBOR_BEFORE_STARVATION, SAFE_NEIGHBOR,
};

#[derive(Clone, Copy, PartialEq)]
pub enum LifeState {
    Alive,
    Dead,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CellType {
    Core,
    Exploration
}

#[derive(Clone, Copy, PartialEq)]
enum ExpansionDirection {
    Top,
    Bottom,
    Right,
    Left,
    TopRight,
    TopLeft,
    BottomLeft,
    BottomRight
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub id: i32,
    pub loc: Int2D,
    pub state: LifeState,
    pub cell_type: CellType,
    pub resource_amount: f32
}

impl Cell {
    pub fn new(
        id: i32,
        loc: Int2D,
        cell_type: CellType,
        resource_amount: f32,
    ) -> Cell {
        Cell {
            id,
            loc,
            state: LifeState::Alive,
            cell_type,
            resource_amount,
        }
    }

    pub fn get_type(&self) -> CellType {
        self.cell_type
    }

    #[allow(dead_code)]
    pub fn as_agent(self) -> Box<dyn Agent> {
        Box::new(self)
    }

    pub fn get_neighborhood_pos(&self, state: &FungiState) -> Vec<Int2D> {
        let mut neighbors = Vec::new();
        let loc = self.loc;

        for dx in -1..=1 {
            for dy in -1..=1 {
                let new_loc = Int2D{ x: loc.x + dx, y: loc.y + dy };
                if dx == 0 && dy == 0 {
                    continue;
                } else if new_loc.x < 0 || new_loc.x >= state.dim.0 as i32 {
                    continue;
                } else if new_loc.y < 0 || new_loc.y >= state.dim.1 as i32 {
                    continue;
                }

                neighbors.push(new_loc);
            }
        }
        neighbors
    }

    fn expansion_to_loc(&self, ed: ExpansionDirection) -> Int2D {
        match ed {
            ExpansionDirection::Top => Int2D{ x: self.loc.x, y: self.loc.y + 1},
            ExpansionDirection::Bottom => Int2D{ x: self.loc.x, y: self.loc.y - 1},
            ExpansionDirection::Right => Int2D{ x: self.loc.x + 1, y: self.loc.y},
            ExpansionDirection::Left => Int2D{ x: self.loc.x - 1, y: self.loc.y},
            ExpansionDirection::TopRight => Int2D{ x: self.loc.x + 1, y: self.loc.y + 1},
            ExpansionDirection::BottomLeft => Int2D{ x: self.loc.x - 1, y: self.loc.y - 1},
            ExpansionDirection::TopLeft => Int2D{ x: self.loc.x - 1, y: self.loc.y + 1},
            ExpansionDirection::BottomRight => Int2D{ x: self.loc.x + 1, y: self.loc.y - 1},
        }
    }

    fn decide_core_development(&mut self, state: &mut FungiState){
        if self.state == LifeState::Dead {
            return;
        }

        let mut empty_cells = Vec::new();
        for loc in self.get_neighborhood_pos(state) {
            if  state.cell_grid.get_objects(&loc) == None {
                empty_cells.push(loc);
            }
        }

        // println!("Cell {} is deciding to develop into a new cell.\n", self.id);
        // println!("Empty cells: {:?}\n", empty_cells.len());
        
        let mut rng = rand::thread_rng();

        if empty_cells.len() == 8 {
            // println!("Cell {} is creating a new cell at random loc.\n", self.id);

            let new_loc = empty_cells[rng.gen_range(0..empty_cells.len())];

            let new_cell = Cell::new(
                state.get_next_id(),
                new_loc,
                CellType::Core,
                self.resource_amount,
            );
            self.resource_amount = 0.0;
            state.new_cells.push(new_cell);
            return;
        }

        let has_top = !empty_cells.contains(&self.expansion_to_loc(ExpansionDirection::Top));
        let has_bottom = !empty_cells.contains(&self.expansion_to_loc(ExpansionDirection::Bottom));
        let has_left = !empty_cells.contains(&self.expansion_to_loc(ExpansionDirection::Left));
        let has_right = !empty_cells.contains(&self.expansion_to_loc(ExpansionDirection::Right));
        let has_top_left = !empty_cells.contains(&self.expansion_to_loc(ExpansionDirection::TopLeft));
        let has_top_right = !empty_cells.contains(&self.expansion_to_loc(ExpansionDirection::TopRight));
        let has_bottom_left = !empty_cells.contains(&self.expansion_to_loc(ExpansionDirection::BottomLeft));
        let has_bottom_right = !empty_cells.contains(&self.expansion_to_loc(ExpansionDirection::BottomRight));

        let neighbour_count = 8 - empty_cells.len();
        let mut deactivation_prob: f32 = neighbour_count as f32 * 0.1;

        if deactivation_prob > MAX_NEIGHBOR_BEFORE_STARVATION {
            deactivation_prob = 0.95;
        } else if deactivation_prob <= SAFE_NEIGHBOR {
            deactivation_prob = 0.01;
        }

        if rng.gen::<f32>() < deactivation_prob {
            // println!("Cell {} is deactivating.\n", self.id);
            self.state = LifeState::Dead;
            return;
        }

        let mut top_prob: f32 = 0.0;
        let mut bottom_prob: f32 = 0.0;
        let mut left_prob: f32 = 0.0;
        let mut right_prob: f32 = 0.0;
        let mut top_right_prob: f32 = 0.0;
        let mut top_left_prob: f32 = 0.0;
        let mut bottom_right_prob: f32 = 0.0;
        let mut bottom_left_prob: f32 = 0.0;

        if has_top {
            bottom_prob += DIRECT_OPPOSITE_CHANCE;
            bottom_left_prob += BROAD_OPPOSITE_CHANCE;
            bottom_right_prob += BROAD_OPPOSITE_CHANCE;
        }

        if has_bottom {
            top_prob += DIRECT_OPPOSITE_CHANCE;
            top_left_prob += BROAD_OPPOSITE_CHANCE;
            top_right_prob += BROAD_OPPOSITE_CHANCE;
        }

        if has_left {
            right_prob += DIRECT_OPPOSITE_CHANCE;
            top_right_prob += BROAD_OPPOSITE_CHANCE;
            bottom_right_prob += BROAD_OPPOSITE_CHANCE;
        }

        if has_right {
            left_prob += DIRECT_OPPOSITE_CHANCE;
            top_left_prob += BROAD_OPPOSITE_CHANCE;
            bottom_left_prob += BROAD_OPPOSITE_CHANCE;
        }

        if has_top_right {
            bottom_left_prob += DIRECT_OPPOSITE_CHANCE;
            left_prob += BROAD_OPPOSITE_CHANCE;
            bottom_prob += BROAD_OPPOSITE_CHANCE;
        }

        if has_top_left {
            bottom_right_prob += DIRECT_OPPOSITE_CHANCE;
            right_prob += BROAD_OPPOSITE_CHANCE;
            bottom_prob += BROAD_OPPOSITE_CHANCE;
        }

        if has_bottom_right {
            top_left_prob += DIRECT_OPPOSITE_CHANCE;
            left_prob += BROAD_OPPOSITE_CHANCE;
            top_prob += BROAD_OPPOSITE_CHANCE;
        }

        if has_bottom_left {
            top_right_prob += DIRECT_OPPOSITE_CHANCE;
            right_prob += BROAD_OPPOSITE_CHANCE;
            top_prob += BROAD_OPPOSITE_CHANCE;
        }

        if has_top {
            top_prob = 0.0;
        }

        if has_bottom {
            bottom_prob = 0.0;
        }

        if has_right {
            right_prob = 0.0;
        }

        if has_left {
            left_prob = 0.0;
        }

        if has_top_right {
            top_right_prob = 0.0;
        }
        
        if has_top_left {
            top_left_prob = 0.0;
        }
        
        if has_bottom_right {
            bottom_right_prob = 0.0;
        }
        
        if has_bottom_left {
            bottom_left_prob = 0.0;
        }
        
        let total = top_prob
            + bottom_prob
            + left_prob
            + right_prob
            + top_right_prob
            + top_left_prob
            + bottom_right_prob
            + bottom_left_prob;

        if total < 0.1 {
            // println!("Cell {} is deactivating.\n", self.id);
            self.state = LifeState::Dead;
            return;
        }

        let child_resource = self.resource_amount * 0.9;
        self.resource_amount -= child_resource;
        let random = rng.gen_range(0.0..total);

        let mut threshold = top_prob;
        if random < threshold { // TOP
            let new_loc = self.expansion_to_loc(ExpansionDirection::Top);
            let new_cell = Cell::new(
                state.get_next_id(),
                new_loc,
                CellType::Core,
                child_resource,
            );
            
            state.new_cells.push(new_cell);
            return;
        }

        threshold += bottom_prob;
        if random < threshold { // BOTTOM
            let new_loc = self.expansion_to_loc(ExpansionDirection::Bottom);
            let new_cell = Cell::new(
                state.get_next_id(),
                new_loc,
                CellType::Core,
                child_resource,
            );
            
            state.new_cells.push(new_cell);
            return;
        }

        threshold += left_prob;
        if random < threshold { // LEFT
            let new_loc = self.expansion_to_loc(ExpansionDirection::Left);
            let new_cell = Cell::new(
                state.get_next_id(),
                new_loc,
                CellType::Core,
                child_resource,
            );
            
            state.new_cells.push(new_cell);
            return;
        }

        threshold += right_prob;
        if random < threshold { // RIGHT
            let new_loc = self.expansion_to_loc(ExpansionDirection::Right);
            let new_cell = Cell::new(
                state.get_next_id(),
                new_loc,
                CellType::Core,
                child_resource,
            );
            
            state.new_cells.push(new_cell);
            return;
        }

        threshold += top_right_prob;
        if random < threshold { // TOP RIGHT
            let new_loc = self.expansion_to_loc(ExpansionDirection::TopRight);
            let new_cell = Cell::new(
                state.get_next_id(),
                new_loc,
                CellType::Core,
                child_resource,
            );
            
            state.new_cells.push(new_cell);
            return;
        }

        threshold += top_left_prob;
        if random < threshold { // TOP LEFT
            let new_loc = self.expansion_to_loc(ExpansionDirection::TopLeft);
            let new_cell = Cell::new(
                state.get_next_id(),
                new_loc,
                CellType::Core,
                child_resource,
            );
            
            state.new_cells.push(new_cell);
            return;
        }

        threshold += bottom_right_prob;
        if random < threshold { // BOTTOM RIGHT
            let new_loc = self.expansion_to_loc(ExpansionDirection::BottomRight);
            let new_cell = Cell::new(
                state.get_next_id(),
                new_loc,
                CellType::Core,
                child_resource,
            );
            
            state.new_cells.push(new_cell);
            return;
        }

        threshold += bottom_left_prob;
        if random < threshold { // BOTTOM LEFT
            let new_loc = self.expansion_to_loc(ExpansionDirection::BottomLeft);
            let new_cell = Cell::new(
                state.get_next_id(),
                new_loc,
                CellType::Core,
                child_resource,
            );
            
            state.new_cells.push(new_cell);
            return;
        }
    }

    fn decide_exploration_development(&mut self, _state: &mut FungiState) {
        panic!("Exploration development not implemented yet.")
    }

    pub fn decide_development(&mut self, state: &mut FungiState) {
        if self.state == LifeState::Dead {
            return;
        }

        if self.cell_type == CellType::Core {
            self.decide_core_development(state)
        } else {
            self.decide_exploration_development(state)
        }
    }
}

impl Agent for Cell {
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any_mut().downcast_mut::<FungiState>().unwrap();

        self.decide_development(state);
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
