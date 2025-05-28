use crate::model::age::Age;
use crate::model::education::Education;
use crate::model::gender::Gender;
use crate::model::status::Status;
use crate::model::{cell::*, state::FakeNewsState};
use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::ecs::system::Resource;
use krabmaga::bevy::prelude::Commands;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::*;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::AgentRender;
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;

use super::vis_cell::VisCell;

#[derive(Clone, Resource)]
pub struct VisState;

impl VisualizationState<FakeNewsState> for VisState {
    fn on_init(
        &self,
        _commands: &mut Commands,
        _sprite_render_factory: &mut AssetHandleFactoryResource,
        _state: &mut FakeNewsState,
        _schedule: &mut Schedule,
        _sim: &mut SimulationDescriptor,
    ) {
    }

    fn get_agent_render(
        &self,
        agent: &Box<dyn Agent>,
        _state: &FakeNewsState,
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
        let state = state
            .as_any()
            .downcast_ref::<FakeNewsState>()
            .unwrap();
        match state.field.get(&Cell {
                id: agent_render.get_id(),
                loc: Real2D { x: 0., y: 0. },
                status: Status::Susceptible,
                education: Education::Primary,
                gender: Gender::Male,
                age: Age::From00to09
        }) {
            Some(matching_agent) => Some(Box::new(*matching_agent)),
            None => None,
        }
    }
}
