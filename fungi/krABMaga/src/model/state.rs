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

use super::cell::{Cell, CellType};

pub struct FungiState{
    pub step: u64,
    pub next_id: i32,
    pub dim: (i32, i32),
    pub resource_grid: DenseNumberGrid2D<i32>,
    pub resource_num_clusters: i32,
    pub resource_cluster_radius: i32,
    pub resource_min_val: i32,
    pub resource_max_val: i32,
    pub resource_min_num_cells: i32,
    pub resource_max_num_cells: i32,
    pub cell_grid: DenseGrid2D<Cell>,
    pub initial_num_cells: i32,
    pub new_cells: Vec<Cell>,
}

impl FungiState {
    pub fn new(dim: (i32, i32)) -> FungiState {
        FungiState {
            step: 0,
            next_id: 1,
            dim,
            resource_grid: DenseNumberGrid2D::new(dim.0, dim.1),
            resource_num_clusters: 10,
            resource_cluster_radius: 5,
            resource_min_val: 50,
            resource_max_val: 100,
            resource_min_num_cells: 5,
            resource_max_num_cells: 15,
            cell_grid: DenseGrid2D::new(dim.0, dim.1),
            initial_num_cells: 1,
            new_cells: Vec::new(),
        }
    }

    pub fn get_next_id(&mut self) -> i32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    // pub fn get_cell(&self, loc: &Int2D) -> Option<Vec<Cell>> {
    //     if loc.x < 0 || loc.x >= self.dim.0 || loc.y < 0 || loc.y >= self.dim.1 {
    //         return None;
    //     }
    //     self.cell_grid.get_objects(&loc)
    // }
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
        if step != 0 {
            // Update the resource grid 
            self.resource_grid.apply_to_all_values(
                |v| { *v },
                GridOption::READWRITE,
            );

            // Update the cell grid
            self.cell_grid.apply_to_all_values(
                |_loc, cell| { Some(*cell) },
                GridOption::READWRITE,
            );
        }

        self.resource_grid.lazy_update();
        self.cell_grid.lazy_update();

        self.step = step;
    }

    fn before_step(&mut self, _schedule: &mut Schedule) {
        self.new_cells.clear();
    }

    fn after_step(&mut self, schedule: &mut Schedule) {
        for cell in self.new_cells.iter() {
            self.cell_grid.set_object_location(*cell, &cell.loc);
            schedule.schedule_repeating(Box::new(*cell), schedule.time + 1.0, 0);
        }

        // Add the resource grid to the plot
        let mut total_resources = 0;
        for x in 0..self.dim.0 {
            for y in 0..self.dim.1 {
                total_resources += self.resource_grid.get_value(&Int2D { x, y }).unwrap_or(0);
            }
        }

        let agents = schedule.get_all_events();
        let mut total_cells = 0;
        let mut total_core_cells = 0;
        let mut total_exploration_cells = 0;

        for n in agents {
            if let Some(cell) = n.downcast_ref::<Cell>() {
                total_cells += 1;
                if cell.get_type() == CellType::Core {
                    total_core_cells += 1;
                } else if cell.get_type() == CellType::Exploration {
                    total_exploration_cells += 1;
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

    // Generate random clusters of resources
    // Each cluster will have a random number of cells with a random value
    for _ in 0..state.resource_num_clusters {
        let center_x = rng.gen_range(0..state.dim.0) as i32;
        let center_y = rng.gen_range(0..state.dim.1) as i32;
        let num_cells = rng.gen_range(state.resource_min_num_cells..=state.resource_max_num_cells);

        for _ in 0..num_cells {
            let dx = rng.gen_range(-state.resource_cluster_radius..=state.resource_cluster_radius); // [-state.cluster_radius, state.cluster_radius]
            let dy = rng.gen_range(-state.resource_cluster_radius..=state.resource_cluster_radius); // [-state.cluster_radius, state.cluster_radius]

            let x = center_x + dx;
            let y = center_y + dy;

            if x >= 0 && x < state.dim.0 && y >= 0 && y < state.dim.1 as i32 {

                let xi = x as usize;
                let yi = y as usize;

                // Check if the cell is already occupied
                if state.resource_grid.get_value(&Int2D { x: xi as i32, y: yi as i32 }).unwrap_or(0) != 0 {
                    continue;
                }

                // Set the value of the cell to a random value between min_val and max_val
                state.resource_grid.set_value_location(
                    rng.gen_range(state.resource_min_val..=state.resource_max_val),
                    &Int2D { x: xi as i32, y: yi as i32 }
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
        // Check if the cell is already occupied        
        let cell = Cell::new(
            id,
            loc,
            CellType::Core,
            1000.0
        );
        state.cell_grid.set_object_location(cell, &loc);

        schedule.schedule_repeating(Box::new(cell), 0., 0);
    }
}
