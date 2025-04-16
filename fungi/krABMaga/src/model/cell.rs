use core::{fmt, panic};
use std::hash::{Hash, Hasher};
use std::vec;

use krabmaga::engine::{agent::Agent, location::Int2D, state::State, schedule::ScheduleOptions};
use krabmaga::rand;
use krabmaga::rand::Rng;

use super::state::{FungiState, StepType};
use crate::{
    BROAD_OPPOSITE_CHANCE, CONSUME_AMOUNT, DIRECT_OPPOSITE_CHANCE, EXPLORATION_BROAD_OPPOSITE_CHANCE, EXPLORATION_DIRECT_OPPOSITE_CHANCE, EXPLORATION_MAX_NEIGHBOR_BEFORE_STARVATION, EXPLORATION_SAFE_NEIGHBOR, HARVEST_AMOUNT, MAX_NEIGHBOR_BEFORE_STARVATION, SAFE_NEIGHBOR
};

#[derive(Clone, Copy, PartialEq, Debug)]
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
    pub resource_amount: f32,

    // Exploration
    is_land_locked: bool,
    checked_for_resources: bool,
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
            is_land_locked: false,
            checked_for_resources: false,
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

    fn sees_resource(&self, state: &FungiState) -> Option<Int2D> {
        for loc in vec![
            self.expansion_to_loc(ExpansionDirection::Top),
            self.expansion_to_loc(ExpansionDirection::Bottom),
            self.expansion_to_loc(ExpansionDirection::Left),
            self.expansion_to_loc(ExpansionDirection::Right),
            self.expansion_to_loc(ExpansionDirection::TopRight),
            self.expansion_to_loc(ExpansionDirection::TopLeft),
            self.expansion_to_loc(ExpansionDirection::BottomRight),
            self.expansion_to_loc(ExpansionDirection::BottomLeft)
        ] {
            if let Some(resource) = state.get_resource(&loc) {
                if resource > 0.0 {
                    return Some(loc);
                }
            }
        }

        None
    }

    fn decide_core_development(&mut self, state: &mut FungiState){
        let mut empty_cells = Vec::new();
        let neighborhood_pos = self.get_neighborhood_pos(state);
        for loc in &neighborhood_pos {
            if  state.cell_grid.get_objects(&loc) == None {
                empty_cells.push(*loc);
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

        let neighbour_count = neighborhood_pos.len() - empty_cells.len();
        let mut deactivation_prob: f32 = neighbour_count as f32 * 0.1;

        if deactivation_prob > MAX_NEIGHBOR_BEFORE_STARVATION {
            deactivation_prob = 0.95;
        } else if deactivation_prob <= SAFE_NEIGHBOR {
            deactivation_prob = 0.01;
        }

        if rng.gen::<f32>() < deactivation_prob {
            // print!("Cell {} is deactivating.\n", self.id);
            if neighbour_count > 1 || self.sees_resource(state).is_none() {
                // println!("Neighbour count: {} or no food", neighbour_count);
                self.state = LifeState::Dead;
                return;
            } else if self.resource_amount < 0.5 {
                // println!("Not enough resources: {}", self.resource_amount);
                self.state = LifeState::Dead;
                return;
            }
            // println!("Cell {} is exploring.\n", self.id);

            let child_resource = self.resource_amount;
            self.resource_amount = 0.0;

            if has_top {
                let new_loc = self.expansion_to_loc(ExpansionDirection::Bottom);
                let new_cell = Cell::new(
                    state.get_next_id(),
                    new_loc,
                    CellType::Exploration,
                    child_resource,
                );
                
                state.new_cells.push(new_cell);
                return;
            }

            if has_bottom {
                let new_loc = self.expansion_to_loc(ExpansionDirection::Top);
                let new_cell = Cell::new(
                    state.get_next_id(),
                    new_loc,
                    CellType::Exploration,
                    child_resource,
                );
                
                state.new_cells.push(new_cell);
                return;
            }

            if has_left {
                let new_loc = self.expansion_to_loc(ExpansionDirection::Right);
                let new_cell = Cell::new(
                    state.get_next_id(),
                    new_loc,
                    CellType::Exploration,
                    child_resource,
                );
                
                state.new_cells.push(new_cell);
                return;
            }

            if has_right {
                let new_loc = self.expansion_to_loc(ExpansionDirection::Left);
                let new_cell = Cell::new(
                    state.get_next_id(),
                    new_loc,
                    CellType::Exploration,
                    child_resource,
                );
                
                state.new_cells.push(new_cell);
                return;
            }

            if has_top_right {
                let new_loc = self.expansion_to_loc(ExpansionDirection::BottomLeft);
                let new_cell = Cell::new(
                    state.get_next_id(),
                    new_loc,
                    CellType::Exploration,
                    child_resource,
                );
                
                state.new_cells.push(new_cell);
                return;
            }

            if has_top_left {
                let new_loc = self.expansion_to_loc(ExpansionDirection::BottomRight);
                let new_cell = Cell::new(
                    state.get_next_id(),
                    new_loc,
                    CellType::Exploration,
                    child_resource,
                );
                
                state.new_cells.push(new_cell);
                return;
            }

            if has_bottom_right {
                let new_loc = self.expansion_to_loc(ExpansionDirection::TopLeft);
                let new_cell = Cell::new(
                    state.get_next_id(),
                    new_loc,
                    CellType::Exploration,
                    child_resource,
                );
                
                state.new_cells.push(new_cell);
                return;
            }

            if has_bottom_left {
                let new_loc = self.expansion_to_loc(ExpansionDirection::TopRight);
                let new_cell = Cell::new(
                    state.get_next_id(),
                    new_loc,
                    CellType::Exploration,
                    child_resource,
                );
                
                state.new_cells.push(new_cell);
                return;
            }

            panic!("Invalid cell expansion. No empty cells found.");
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

    fn decide_exploration_development(&mut self, state: &mut FungiState) {
        if self.is_land_locked {
            self.state = LifeState::Dead;
            return;
        }

        if let Some(resource_loc) = self.sees_resource(state) {
            let cell = state.get_cell(&resource_loc);
            if cell.is_none() {
                let child_resource = self.resource_amount;
                self.resource_amount = 0.0;
                let new_cell = Cell::new(
                    state.get_next_id(),
                    resource_loc,
                    CellType::Core,
                    child_resource,
                );
                state.new_cells.push(new_cell);
                return;
            }
        }


        let mut empty_cells = Vec::new();
        let neighborhood_pos = self.get_neighborhood_pos(state);
        for loc in &neighborhood_pos {
            if  state.cell_grid.get_objects(&loc) == None {
                empty_cells.push(*loc);
            }
        }

        let mut rng = rand::thread_rng();
        
        let has_top = !empty_cells.contains(&self.expansion_to_loc(ExpansionDirection::Top));
        let has_bottom = !empty_cells.contains(&self.expansion_to_loc(ExpansionDirection::Bottom));
        let has_left = !empty_cells.contains(&self.expansion_to_loc(ExpansionDirection::Left));
        let has_right = !empty_cells.contains(&self.expansion_to_loc(ExpansionDirection::Right));
        let has_top_left = !empty_cells.contains(&self.expansion_to_loc(ExpansionDirection::TopLeft));
        let has_top_right = !empty_cells.contains(&self.expansion_to_loc(ExpansionDirection::TopRight));
        let has_bottom_left = !empty_cells.contains(&self.expansion_to_loc(ExpansionDirection::BottomLeft));
        let has_bottom_right = !empty_cells.contains(&self.expansion_to_loc(ExpansionDirection::BottomRight));

        let neighbour_count = neighborhood_pos.len() - empty_cells.len();
        let mut deactivation_prob: f32 = neighbour_count as f32 * 0.1;

        if deactivation_prob > EXPLORATION_MAX_NEIGHBOR_BEFORE_STARVATION {
            deactivation_prob = 0.95;
        } else if deactivation_prob <= EXPLORATION_SAFE_NEIGHBOR {
            deactivation_prob = 0.01;
        }

        if rng.gen::<f32>() < deactivation_prob {
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
            bottom_prob += EXPLORATION_DIRECT_OPPOSITE_CHANCE;
            bottom_left_prob += EXPLORATION_BROAD_OPPOSITE_CHANCE;
            bottom_right_prob += EXPLORATION_BROAD_OPPOSITE_CHANCE;
        }

        if has_bottom {
            top_prob += EXPLORATION_DIRECT_OPPOSITE_CHANCE;
            top_left_prob += EXPLORATION_BROAD_OPPOSITE_CHANCE;
            top_right_prob += EXPLORATION_BROAD_OPPOSITE_CHANCE;
        }

        if has_left {
            right_prob += EXPLORATION_DIRECT_OPPOSITE_CHANCE;
            top_right_prob += EXPLORATION_BROAD_OPPOSITE_CHANCE;
            bottom_right_prob += EXPLORATION_BROAD_OPPOSITE_CHANCE;
        }

        if has_right {
            left_prob += EXPLORATION_DIRECT_OPPOSITE_CHANCE;
            top_left_prob += EXPLORATION_BROAD_OPPOSITE_CHANCE;
            bottom_left_prob += EXPLORATION_BROAD_OPPOSITE_CHANCE;
        }

        if has_top_right {
            bottom_left_prob += EXPLORATION_DIRECT_OPPOSITE_CHANCE;
            left_prob += EXPLORATION_BROAD_OPPOSITE_CHANCE;
            bottom_prob += EXPLORATION_BROAD_OPPOSITE_CHANCE;
        }

        if has_top_left {
            bottom_right_prob += EXPLORATION_DIRECT_OPPOSITE_CHANCE;
            right_prob += EXPLORATION_BROAD_OPPOSITE_CHANCE;
            bottom_prob += EXPLORATION_BROAD_OPPOSITE_CHANCE;
        }

        if has_bottom_right {
            top_left_prob += EXPLORATION_DIRECT_OPPOSITE_CHANCE;
            left_prob += EXPLORATION_BROAD_OPPOSITE_CHANCE;
            top_prob += EXPLORATION_BROAD_OPPOSITE_CHANCE;
        }

        if has_bottom_left {
            top_right_prob += EXPLORATION_DIRECT_OPPOSITE_CHANCE;
            right_prob += EXPLORATION_BROAD_OPPOSITE_CHANCE;
            top_prob += EXPLORATION_BROAD_OPPOSITE_CHANCE;
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

        let child_resource = self.resource_amount;
        self.resource_amount = 0.0;
        let random = rng.gen_range(0.0..total);

        let mut threshold = top_prob;
        if random < threshold { // TOP
            let new_loc = self.expansion_to_loc(ExpansionDirection::Top);
            let new_cell = Cell::new(
                state.get_next_id(),
                new_loc,
                CellType::Exploration,
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
                CellType::Exploration,
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
                CellType::Exploration,
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
                CellType::Exploration,
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
                CellType::Exploration,
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
                CellType::Exploration,
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
                CellType::Exploration,
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
                CellType::Exploration,
                child_resource,
            );
            
            state.new_cells.push(new_cell);
            return;
        }
    }

    pub fn grow_step(&mut self, state: &mut FungiState) {
        if self.state == LifeState::Dead {
            return;
        }

        if self.cell_type == CellType::Core {
            self.decide_core_development(state)
        }
    }

    pub fn explore_step(&mut self, state: &mut FungiState) {
        if self.state == LifeState::Dead {
            return;
        }

        if self.cell_type == CellType::Exploration {
            self.decide_exploration_development(state)
        }
    }

    fn harvest_resource(&mut self, state: &mut FungiState) {
        if self.checked_for_resources {
            return;
        }

        if let Some(resource) = state.get_resource(&self.loc) {
            if resource > 0.0 {
                if resource < HARVEST_AMOUNT {
                    self.resource_amount += resource;
                    state.resource_grid.set_value_location(0.0, &self.loc);
                } else {
                    self.resource_amount += HARVEST_AMOUNT;
                    state.resource_grid.set_value_location(resource - HARVEST_AMOUNT, &self.loc);
                }
            } else {
                self.checked_for_resources = true;
            }
        }
    }

    pub fn add_resource(&mut self, amount: f32) {
        self.resource_amount += amount;
    }

    fn consume_resource(&mut self) {
        if self.state == LifeState::Dead {
            return;
        }

        if self.resource_amount > CONSUME_AMOUNT {
            self.resource_amount -= CONSUME_AMOUNT;
        } else {
            // Starve
            self.state = LifeState::Dead;
        }
    }

    fn get_neighbors(&self, state: &FungiState) -> Vec<Cell> {
        let mut neighbors = Vec::new();
        for loc in self.get_neighborhood_pos(state) {
            if let Some(cells) = state.cell_grid.get_objects(&loc) {
                for cell in cells {
                    if cell.id != self.id {
                        neighbors.push(cell);
                    }
                }
            }
        }

        neighbors
    }

    fn get_share_amount(&mut self) -> f32 {
        if self.cell_type == CellType::Core {
            
            if self.resource_amount < 1.0 {
                return 0.0
            } else if self.resource_amount < 10.0 {
                let share_amount = self.resource_amount / 2.0;
                self.resource_amount -= share_amount;
                return share_amount;
            } else {
                let share_amount = self.resource_amount * 0.8;
                self.resource_amount -= share_amount;
                return share_amount;
            }

        } else {
            let share_amount = self.resource_amount;
            self.resource_amount = 0.0;
            return share_amount;
        }
    }

    fn revive(&mut self, neighbors: &Vec<Cell>) {
        if (self.cell_type == CellType::Exploration
            && neighbors.len() == 1
            && self.resource_amount > 0.5
            && !self.is_land_locked)
            || self.resource_amount > 5.0 { // Revive
            self.state = LifeState::Alive;
        }
    }

    fn share_resource(&mut self, state: &mut FungiState, neighbors: &Vec<Cell>) {
        if self.cell_type == CellType::Exploration
            && neighbors.len() == 1 && !self.is_land_locked {
            return;
        }

        let share_amount = self.get_share_amount();
        if share_amount == 0.0 {
            return;
        }

        let share_amount_per_neighbor = share_amount / neighbors.len() as f32;
        
        if self.cell_type == CellType::Core {
            let exploratory_neighbors: Vec<Cell> = neighbors.clone()
                .into_iter()
                .filter(|neighbor| neighbor.cell_type == CellType::Exploration)
                .collect();

            if exploratory_neighbors.len() > 0 {
                let share_amount_per_exploratory_neighbor = share_amount / exploratory_neighbors.len() as f32;
                for neighbor in exploratory_neighbors {
                    state
                        .update_cells_resource
                        .entry(neighbor.loc)
                        .and_modify(|val| *val += share_amount_per_exploratory_neighbor)
                        .or_insert(share_amount_per_exploratory_neighbor);
                }
            } else {
                for neighbor in neighbors {
                    // println!("Cell {} is sharing {} with neighbor {} at loc x {} y {}", self.id, share_amount_per_neighbor, neighbor.id, neighbor.loc.x, neighbor.loc.y);

                    state
                        .update_cells_resource
                        .entry(neighbor.loc)
                        .and_modify(|val| *val += share_amount_per_neighbor)
                        .or_insert(share_amount_per_neighbor);
                }
            }
        } else {
            for neighbor in neighbors {
                state
                    .update_cells_resource
                    .entry(neighbor.loc)
                    .and_modify(|val| *val += share_amount_per_neighbor)
                    .or_insert(share_amount_per_neighbor);
            } 
        }
    }

    pub fn feed_step(&mut self, state: &mut FungiState) {
        self.harvest_resource(state);
        self.consume_resource();

        let neighbors = self.get_neighbors(state);
        self.share_resource(state, &neighbors);
    }
}

impl Agent for Cell {
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any_mut().downcast_mut::<FungiState>().unwrap();
        
        // println!("{}-> Cell {}, resource {}, state: {:?} \n", state.step, self.id, self.resource_amount, self.state);

        // state.update_cells_resource.push((self.loc, 2.0));

        state.cell_grid.set_object_location(*self, &self.loc);

        // if self.state == LifeState::Dead {
        //     return;
        // }

        match state.get_step_type() {
            StepType::Feed => {
                // println!("BEFORE: {} (FEED) -> Cell {}, resource {}, state: {:?} \n", state.step, self.id, self.resource_amount, self.state);
                self.feed_step(state);
                // println!("AFTER: {} (FEED) -> Cell {}, resource {}, state: {:?} \n", state.step, self.id, self.resource_amount, self.state);
            }
            StepType::Grow => {
                // println!("BEFORE: {} (GROW) -> Cell {}, resource {}, state: {:?}\n", state.step, self.id, self.resource_amount, self.state);
                self.grow_step(state);
                // println!("AFTER: {} (GROW) -> Cell {}, resource {}, state: {:?} \n", state.step, self.id, self.resource_amount, self.state);
            }
            StepType::Explore => {
                // println!("BEFORE: {} (EXPLORE) -> Cell {}, resource {}, state: {:?} \n", state.step, self.id, self.resource_amount, self.state);
                self.explore_step(state);
                // println!("AFTER: {} (EXPLORE) -> Cell {}, resource {}, state: {:?} \n", state.step, self.id, self.resource_amount, self.state);
            }
        }
    }

    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        false
    }

    fn before_step(
            &mut self,
            state: &mut dyn State,
        ) -> Option<Vec<(Box<dyn Agent>, ScheduleOptions)>> {

        let state = state.as_any_mut().downcast_mut::<FungiState>().unwrap();

        if state.get_prev_step_type() == StepType::Feed {
            if state.update_cells_resource.contains_key(&self.loc) {
                let resource = state.update_cells_resource.get(&self.loc).unwrap();
                self.add_resource(*resource);
                state.update_cells_resource.remove(&self.loc);
            }

            if self.state == LifeState::Dead {
                let neighbors = self.get_neighbors(state);
                self.revive(&neighbors);
            }
        }

        None
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
