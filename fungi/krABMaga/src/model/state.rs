use std::any::Any;

use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::grid_option::GridOption;
use krabmaga::*;

use krabmaga::engine::{
    fields::dense_number_grid_2d::DenseNumberGrid2D,
    fields::field::Field, location::Int2D,
    schedule::Schedule, state::State
};
use krabmaga::rand;
use krabmaga::rand::Rng;

use super::core_cell::CoreCell;
use super::exploration_cell::ExplorationCell;
use crate::{
    RESOURCE_CLUSTER_NUM, RESOURCE_MAX_AMOUNT,
    RESOURCE_CLUSTER_RADIUS, RESOURCE_MIN_AMOUNT
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LifeState {
    Alive,
    Dead,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ExpansionDirection {
    Top,
    Bottom,
    Right,
    Left,
    TopRight,
    TopLeft,
    BottomLeft,
    BottomRight
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StepType{
    Grow,
    Feed,
    Explore
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    Core(CoreCell),
    Exploration(ExplorationCell),
}

pub struct FungiState{
    pub step: u64,
    pub next_id: i32,
    pub dim: (i32, i32),
    pub resource_grid: DenseNumberGrid2D<f32>,
    pub cell_grid: DenseGrid2D<Cell>,
    pub initial_num_cells: i32,
    pub new_cells: Vec<Cell>,
    pub update_cells_resource: HashMap<Int2D, f32>,
    pub total_alive_cells: i32,
}

impl FungiState {
    pub fn new(dim: (i32, i32)) -> FungiState {
        FungiState {
            step: 0,
            next_id: 1,
            dim,
            resource_grid: DenseNumberGrid2D::new(dim.0, dim.1),
            cell_grid: DenseGrid2D::new(dim.0, dim.1),
            initial_num_cells: 1,
            new_cells: Vec::new(),
            update_cells_resource: HashMap::new(),
            total_alive_cells: 0
        }
    }

    pub fn get_next_id(&mut self) -> i32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn get_cell(&self, loc: &Int2D) -> Option<Vec<Cell>> {
        if loc.x < 0 || loc.x >= self.dim.0 || loc.y < 0 || loc.y >= self.dim.1 {
            return None;
        }
        self.cell_grid.get_objects(&loc)
    }

    pub fn get_resource(&self, loc: &Int2D) -> Option<f32> {
        if loc.x < 0 || loc.x >= self.dim.0 || loc.y < 0 || loc.y >= self.dim.1 {
            return None;
        }
        self.resource_grid.get_value(&loc)
    }

    pub fn get_step_type(&self) -> StepType {
        match self.step % 5 {
            0 => StepType::Grow,
            1 => StepType::Feed,
            _ => StepType::Explore,
        }
    }

    pub fn get_prev_step_type(&self) -> StepType {
        if self.step == 0 {
            return StepType::Grow;
        }

        match (self.step - 1) % 5 {
            0 => StepType::Grow,
            1 => StepType::Feed,
            _ => StepType::Explore,
        }
    }
}

impl State for FungiState {
    fn init(&mut self, schedule: &mut Schedule) {
        self.reset();

        generate_resources(self);
        generate_core_cells(self, schedule);

        addplot!(
            String::from("Resources over time"),
            String::from("Steps"),
            String::from("Resources"),
            true
        );

        addplot!(
            String::from("Cells over time"),
            String::from("Steps"),
            String::from("Cells"),
            true
        );
    }

    fn update(&mut self, step: u64) {
        self.step = step;

        if step != 0 {
            // Update the resource grid 
            self.resource_grid.apply_to_all_values(
                |v| { *v },
                GridOption::READWRITE,
            );
        }

        self.resource_grid.lazy_update();
        self.cell_grid.lazy_update();
    }

    fn before_step(&mut self, _schedule: &mut Schedule) {
        self.new_cells.clear();

        if self.get_step_type() == StepType::Grow {
            self.update_cells_resource.clear();
        }
    }

    fn after_step(&mut self, schedule: &mut Schedule) {
        // Add new cells to the grid
        for cell in self.new_cells.iter() {
            match cell {
                Cell::Core(core_cell) => {
                    self.cell_grid.set_object_location(*cell, &core_cell.loc);
                    schedule.schedule_repeating(Box::new(*core_cell), schedule.time + 1.0, 0);
                },
                Cell::Exploration(exploration_cell) => {
                    self.cell_grid.set_object_location(*cell, &exploration_cell.loc);
                    schedule.schedule_repeating(Box::new(*exploration_cell), schedule.time + 1.0, 0);
                },
            };
        }

        // Calculate the total resources in the grid
        let mut total_resources = 0.0;
        for x in 0..self.dim.0 {
            for y in 0..self.dim.1 {
                total_resources += self.resource_grid.get_value(&Int2D { x, y }).unwrap_or(0.0);
            }
        }

        let agents = schedule.get_all_events();
        let mut total_cells = 0;
        let mut total_core_cells = 0;
        let mut total_exploration_cells = 0;

        self.total_alive_cells = 0;

        for n in agents {
            if let Some(core_cell) = n.downcast_ref::<CoreCell>() {
                total_cells += 1;
                total_core_cells += 1;

                if core_cell.state == LifeState::Alive {
                    self.total_alive_cells += 1;
                }
            } else if let Some(exploration_cell) = n.downcast_ref::<ExplorationCell>() {
                total_cells += 1;
                total_exploration_cells += 1;

                if exploration_cell.state == LifeState::Alive {
                    self.total_alive_cells += 1;
                }
            }
        }

        plot!(
            String::from("Resources over time"),
            String::from("Resources"),
            self.step as f64,
            total_resources as f64
        );

        plot!(
            String::from("Cells over time"),
            String::from("Total Cells"),
            self.step as f64,
            total_cells as f64
        );

        plot!(
            String::from("Cells over time"),
            String::from("Core Cells"),
            self.step as f64,
            total_core_cells as f64
        );

        plot!(
            String::from("Cells over time"),
            String::from("Exploration Cells"),
            self.step as f64,
            total_exploration_cells as f64
        );
    }

    fn reset(&mut self) {
        self.step = 0;
        self.resource_grid = DenseNumberGrid2D::new(self.dim.0, self.dim.1);
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

fn generate_resources(state: &mut FungiState) {
    let mut rng = rand::thread_rng();

    for _ in 0..RESOURCE_CLUSTER_NUM {
        let x = rng.gen_range(RESOURCE_CLUSTER_RADIUS..state.dim.0-RESOURCE_CLUSTER_RADIUS);
        let y = rng.gen_range(RESOURCE_CLUSTER_RADIUS..state.dim.1-RESOURCE_CLUSTER_RADIUS);
        let radius = rng.gen_range(1..RESOURCE_CLUSTER_RADIUS);

        for j in (x - radius)..(x + radius) {
            for k in (y - radius)..(y + radius) {
                // Check if the cell is already occupied
                if state.resource_grid.get_value(&Int2D { x: j as i32, y: k as i32 }).unwrap_or(0.0) != 0.0 {
                    continue;
                }

                // Set the value of the cell to a random value between min_val and max_val
                state.resource_grid.set_value_location(
                    rng.gen_range(RESOURCE_MIN_AMOUNT..=RESOURCE_MAX_AMOUNT),
                    &Int2D { x: j as i32, y: k as i32 }
                );
            }
        }
    }
}

fn generate_core_cells(state: &mut FungiState, schedule: &mut Schedule) {
    let mut rng = rand::thread_rng();

    for id in 0..state.initial_num_cells {
        let loc = Int2D {
            x: rng.gen_range(0..state.dim.0),
            y: rng.gen_range(0..state.dim.1),
        };
   
        let cell = CoreCell::new(
            id,
            loc,
            1000.0
        );
        state.cell_grid.set_object_location(Cell::Core(cell), &loc);

        schedule.schedule_repeating(Box::new(cell), 0., 0);
    }
}
