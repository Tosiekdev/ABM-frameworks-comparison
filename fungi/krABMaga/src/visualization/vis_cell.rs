use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::prelude::{Component, Quat, Transform, Visibility};
use krabmaga::engine::agent::Agent;
// use krabmaga::engine::location::Int2D;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::{AgentRender, SpriteType};

// use crate::model::state::FungiState;
use crate::model::cell::{Cell, CellType};

#[derive(Component)]
pub struct VisCell {
    pub id: u32,
}

impl AgentRender for VisCell {
    fn sprite(&self, agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> SpriteType {
        let agent = agent.downcast_ref::<Cell>().unwrap();
        if agent.cell_type == CellType::Core {
            SpriteType::Emoji(String::from("black-square-32"))
        } else {
            SpriteType::Emoji(String::from("gray-square-32"))
        }
    }

    fn location(&self, agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> (f32, f32, f32) {
        let agent = agent.downcast_ref::<Cell>().unwrap();
        (
            agent.loc.x as f32,
            agent.loc.y as f32,
            1.
        )
    }

    fn scale(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> (f32, f32) {
        (1./32., 1./32.)
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
        let (scale_x, scale_y) = self.scale(agent, state);
        let rotation = self.rotation(agent, state);
        let translation = &mut transform.translation;
        translation.x = loc_x;
        translation.y = loc_y;
        translation.z = z;
        transform.scale.x = scale_x;
        transform.scale.y = scale_y;
        transform.rotation = Quat::from_rotation_z(rotation);
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}