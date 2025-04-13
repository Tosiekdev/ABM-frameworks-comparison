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

use crate::model::cell::{Cell, CellType};
use crate::model::state::FungiState;

use super::vis_cell::VisCell;

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
        let agent = agent.downcast_ref::<Cell>().unwrap();
        Some(Box::new(VisCell { id: agent.id as u32}))
    }

    fn get_agent(
        &self,
        agent_render: &Box<dyn AgentRender>,
        state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        let state = state.as_any().downcast_ref::<FungiState>().unwrap();
        match state.cell_grid.get(&Cell::new(
            agent_render.get_id() as i32,
            Int2D { x: 0, y: 0 },
            CellType::Core,
            0.
        )) {
            Some(matching_agent) => Some(Box::new(matching_agent)),
            None => None
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
            let boxed_agent = &cell.as_agent();
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
