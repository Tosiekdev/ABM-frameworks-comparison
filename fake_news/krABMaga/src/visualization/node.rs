use krabmaga::bevy::prelude::{Component, Transform, Visibility};

use crate::model::node::*;
use crate::model::status::Status;
use crate::model::state::FakeNewsState;
use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::{AgentRender, SpriteType};

#[derive(Component)]
pub struct VisNode {
    pub id: u32,
}

impl AgentRender for VisNode {
    fn sprite(&self, agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> SpriteType {
        let agent = agent.downcast_ref::<Node>().unwrap();
        let state = state
            .as_any()
            .downcast_ref::<FakeNewsState>()
            .unwrap();

        let status = if let Some(updated_agent) = state.network.get_object(agent.id) {
            updated_agent.status
        } else {
            agent.status
        };
        match status {
            Status::Susceptible => SpriteType::Emoji(String::from("white-circle")),
            Status::Exposed => SpriteType::Emoji(String::from("green-circle")),
            Status::Infected => SpriteType::Emoji(String::from("red-circle")),
            Status::Doubtful => SpriteType::Emoji(String::from("black-circle")),
            Status::Recovered => SpriteType::Emoji(String::from("blue-circle")),
        }
    }

    fn location(&self, agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> (f32, f32, f32) {
        let state = state
            .as_any()
            .downcast_ref::<FakeNewsState>()
            .unwrap();
        let agent = agent.downcast_ref::<Node>().unwrap();
        let loc = state.field.get_location(*agent);
        match loc {
            Some(loc) => (loc.x, loc.y, 0.),
            None => (agent.loc.x, agent.loc.y, 0.),
        }
    }

    /// Emojis are 64x64, way too big for our simulation
    fn scale(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> (f32, f32) {
        (0.3, 0.3)
    }

    /// No rotation is needed
    fn rotation(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> f32 {
        0.
    }

    /// Simply update the transform based on the location chosen
    fn update(
        &mut self,
        agent: &Box<dyn Agent>,
        transform: &mut Transform,
        state: &Box<&dyn State>,
        _visible: &mut Visibility,
    ) {
        let (loc_x, loc_y, z) = self.location(agent, state);
        let translation = &mut transform.translation;
        translation.x = loc_x;
        translation.y = loc_y;
        translation.z = z;
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}
