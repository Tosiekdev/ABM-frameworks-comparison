use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::ecs::system::Resource;
use krabmaga::bevy::prelude::Commands;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::AgentRender;
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;

use crate::model::cell::{Cell, LifeState};
use crate::model::state::GameOfLifeState;

use super::vis_cell::VisCell;


#[derive(Clone, Resource)]
pub struct VisState;

impl VisualizationState<GameOfLifeState> for VisState {
    fn on_init(
        &self,
        _commands: &mut Commands,
        _sprite_render_factory: &mut AssetHandleFactoryResource,
        _state: &mut GameOfLifeState,
        _schedule: &mut Schedule,
        _sim: &mut SimulationDescriptor,
    ) {
    }

    fn get_agent_render(
        &self,
        agent: &Box<dyn Agent>,
        _state: &GameOfLifeState,
    ) -> Option<Box<dyn AgentRender>> {
        Some(Box::new(VisCell {
            id: agent.downcast_ref::<Cell>().unwrap().id,
        }))
    }

    fn get_agent(
        &self,
        agent_render: &Box<dyn AgentRender>,
        state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        let state = state.as_any().downcast_ref::<GameOfLifeState>().unwrap();
        match state.cell_grid.get(&Cell {
            id: agent_render.get_id(),
            loc: Real2D { x: 0., y: 0. },
            state: LifeState::Alive,
        }) {
            Some(matching_agent) => Some(Box::new(*matching_agent)),
            None => None,
        }
    }
}
