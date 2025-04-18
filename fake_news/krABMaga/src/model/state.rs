use krabmaga::engine::fields::network::Network;
use krabmaga::engine::fields::{field::Field, field_2d::Field2D};
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::any::Any;

use crate::model::node::{Node, Status};
use crate::INIT_EDGES;

pub struct FakeNewsState {
    pub step: u64,
    pub network: Network<Node, String>,
    pub num_nodes: u32,

    // just for visualization
    pub field: Field2D<Node>,
    pub discretization: f32,
    pub toroidal: bool,
    pub dim: (f32, f32),
}

impl FakeNewsState {
    pub fn new(dim: (f32, f32), num_nodes: u32, d: f32, t: bool) -> FakeNewsState {
        FakeNewsState {
            step: 0,
            network: Network::new(false),
            num_nodes,
            discretization: d,
            field: Field2D::new(dim.0, dim.1, d, t),
            toroidal: t,
            dim,
        }
    }
}

impl State for FakeNewsState {
    fn reset(&mut self) {
        self.step = 0;
        self.field = Field2D::new(self.dim.0, self.dim.1, self.discretization, self.toroidal);
        self.network = Network::new(false);
    }

    fn init(&mut self, schedule: &mut Schedule) {
        self.reset();
        generate_network(self, schedule);
    }

    fn update(&mut self, step: u64) {
        self.field.lazy_update();
        self.network.update();
        self.step = step;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }
}

fn generate_network(state: &mut FakeNewsState, schedule: &mut Schedule) {
    let mut node_set = Vec::new();
    let mut rng = rand::thread_rng();

    for node_id in 0..state.num_nodes {
        let r1: f32 = rng.gen();
        let r2: f32 = rng.gen();

        let init_status: Status = rng.gen();
        
        let node = Node::new(
            node_id,
            Real2D {
                x: state.dim.0 * r1,
                y: state.dim.1 * r2,
            },
            init_status,
        );

        state.field.set_object_location(node, node.loc);
        state.network.add_node(node);
        schedule.schedule_repeating(Box::new(node), 0.0, 0);
        node_set.push(node);
    }

    state.network
        .preferential_attachment_BA(&node_set, INIT_EDGES);
}
