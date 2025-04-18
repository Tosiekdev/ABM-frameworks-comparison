use crate::model::node::Node;
use crate::model::state::FakeNewsState;
use krabmaga::bevy::prelude::*;
use krabmaga::engine::fields::network::{Edge, Network};
use krabmaga::engine::location::Real2D;
use krabmaga::visualization::fields::network::{EdgeRenderInfo, NetworkRender};

impl NetworkRender<Node, String, FakeNewsState> for FakeNewsState {
    fn get_network(state: &FakeNewsState) -> &Network<Node, String> {
        &state.network
    }

    fn get_edge_info(edge: &Edge<String>, network: &Network<Node, String>) -> EdgeRenderInfo {
        EdgeRenderInfo {
            line_color: Color::BLACK,
            line_width: 1.,
            source_loc: network.get_object(edge.u).unwrap().loc,
            target_loc: network.get_object(edge.v).unwrap().loc,
            is_static: true,
        }
    }

    fn get_loc(network: &Network<Node, String>, node: u32) -> Real2D {
        network.get_object(node).unwrap().loc
    }
}
