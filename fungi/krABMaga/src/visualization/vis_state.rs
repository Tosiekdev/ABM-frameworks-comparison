use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::ecs::system::Resource;
use krabmaga::bevy::prelude::Commands;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::{AgentRender, SpriteType};
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::fields::number_grid_2d::BatchRender;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;

use crate::model::core_cell::CoreCell;
use crate::model::exploration_cell::ExplorationCell;
use crate::model::state::{Cell, FungiState};

use super::vis_core_cell::VisCoreCell;
use super::vis_exploration_cell::VisExplorationCell;

#[derive(Clone, Resource)]
pub struct VisState;

impl VisState {
    fn generate_resources(
        state: &FungiState,
        sprite_render_factory: &mut AssetHandleFactoryResource,
        commands: &mut Commands,
        sim: &mut SimulationDescriptor,
    ) {
        state
            .resource_grid
            .render(&mut *sprite_render_factory, commands, sim);
    }
}

impl VisualizationState<FungiState> for VisState {
    fn on_init(
        &self,
        commands: &mut Commands,
        sprite_render_factory: &mut AssetHandleFactoryResource,
        state: &mut FungiState,
        _schedule: &mut Schedule,
        sim: &mut SimulationDescriptor,
    ) {
        Self::generate_resources(state, sprite_render_factory, commands, sim);
    }

    fn get_agent_render(
        &self,
        agent: &Box<dyn Agent>,
        _state: &FungiState,
    ) -> Option<Box<dyn AgentRender>> {
        if let Some(agent) = agent.downcast_ref::<CoreCell>() {
            Some(Box::new(VisCoreCell { id: agent.id as u32}))
        } else if let Some(agent) = agent.downcast_ref::<ExplorationCell>() {
            Some(Box::new(VisExplorationCell { id: agent.id as u32}))
        } else {
            None
        }
    }

    fn get_agent(
        &self,
        agent_render: &Box<dyn AgentRender>,
        state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        let state = state.as_any().downcast_ref::<FungiState>().unwrap();
        if let Some(matching_agent) = state.cell_grid.get(&Cell::Core(CoreCell::new(
            agent_render.get_id() as i32,
            Int2D { x: 0, y: 0 },
            0.
        ))) {
            match matching_agent {
                Cell::Core(matching_agent) => Some(Box::new(matching_agent)),
                _ => None,
            }
        } else  if let Some(matching_agent) = state.cell_grid.get(&&Cell::Exploration(ExplorationCell::new(
            agent_render.get_id() as i32,
            Int2D { x: 0, y: 0 },
            0.
        ))) {
            match matching_agent {
                Cell::Exploration(matching_agent) => Some(Box::new(matching_agent)),
                _ => None,
            }
        } else {
            None
        }
    }

    fn before_render(
        &mut self,
        state: &mut FungiState,
        _schedule: &Schedule,
        commands: &mut Commands,
        asset_factory: &mut AssetHandleFactoryResource,
    ) {
        for cell in &state.new_cells {
            let boxed_agent = match cell {
                Cell::Core(core_cell) => &core_cell.as_agent(),
                Cell::Exploration(exploration_cell) => &exploration_cell.as_agent(),
            };
            let boxed_state = Box::new(state.as_state());
            let cell_vis = self.get_agent_render(boxed_agent, state);
            let SpriteType::Emoji(emoji_code) =
                cell_vis.unwrap().sprite(boxed_agent, &boxed_state);
            let sprite_render = asset_factory.get_emoji_loader(emoji_code);
            self.setup_agent_graphics(
                boxed_agent,
                self.get_agent_render(boxed_agent, state).unwrap(),
                sprite_render,
                commands,
                &boxed_state,
            );
        }
    }
}
