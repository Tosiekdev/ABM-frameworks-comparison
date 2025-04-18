use std::{
    fmt,
    hash::{Hash, Hasher},
};

use krabmaga::engine::state::State;
use krabmaga::engine::{
    agent::Agent,
    fields::field_2d::Location2D,
    location::Real2D
};

use crate::model::state::FakeNewsState;

use super::{age::Age, education::Education, gender::Gender, status::Status};

#[derive(Clone, Copy)]
pub struct Node {
    pub id: u32,
    pub loc: Real2D,
    pub status: Status,
    pub education: Education,
    pub gender: Gender,
    pub age: Age
}

impl Node {
    pub fn new(id: u32, loc: Real2D, init_status: Status, education: Education, gender: Gender, age: Age) -> Self {
        Node {
            id,
            loc,
            status: init_status,
            education,
            gender,
            age
        }
    }
}

impl Agent for Node {
    fn step(&mut self, state: &mut dyn State) {
        let state = state
            .as_any()
            .downcast_ref::<FakeNewsState>()
            .unwrap();

        state.network.update_node(*self);
        state.field.set_object_location(*self, self.loc);
    }
}

impl Hash for Node {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id
    }
}

impl Location2D<Real2D> for Node {
    fn get_location(self) -> Real2D {
        self.loc
    }

    fn set_location(&mut self, loc: Real2D) {
        self.loc = loc;
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} loc {} status {}", self.id, self.loc, self.status)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Status::Susceptible => write!(f, "Susceptible"),
            Status::Exposed => write!(f, "Exposed"),
            Status::Infected => write!(f, "Infected"),
            Status::Doubtful => write!(f, "Doubtful"),
            Status::Recovered => write!(f, "Recovered")
        }
    }
}
