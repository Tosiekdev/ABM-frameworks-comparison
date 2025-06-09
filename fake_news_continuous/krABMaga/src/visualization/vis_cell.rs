use krabmaga::bevy::prelude::{Component, Transform, Visibility};

use crate::model::cell::*;
use crate::model::status::Status;
use crate::model::state::FakeNewsState;
use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::{AgentRender, SpriteType};

#[derive(Component)]
pub struct VisCell {
    pub id: u32,
}

impl AgentRender for VisCell {
    fn sprite(&self, agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> SpriteType {
        let agent = agent.downcast_ref::<Cell>().unwrap();

        match agent.status {
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
        let agent = agent.downcast_ref::<Cell>().unwrap();
        let loc = state.field.get_location(*agent);
        match loc {
            Some(loc) => (loc.x, loc.y, 0.),
            None => (agent.loc.x, agent.loc.y, 0.),
        }
    }

    fn scale(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> (f32, f32) {
        (1./64., 1./64.)
    }

    fn rotation(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> f32 {
        0.
    }

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
