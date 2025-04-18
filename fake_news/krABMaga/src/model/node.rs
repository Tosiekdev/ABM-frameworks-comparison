use core::panic;
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

use krabmaga::rand;
use krabmaga::rand::Rng;

use crate::{model::state::FakeNewsState, ALPHA, BETA, DELTA, GAMMA, PARAM_A, PARAM_B, PARAM_C, THETA};

use super::{
    age::{Age, AGE_DISTRIBUTION},
    education::{Education, EDUCATION_DISTRIBUTION},
    gender::{Gender, GENDER_DISTRIBUTION},
    status::Status
};

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

    fn compute_transition_threshold(&self, to: Status) -> f32 {
        let age_value = AGE_DISTRIBUTION
            .get(&self.status).unwrap()
            .get(&self.age).unwrap();

        let education_value = EDUCATION_DISTRIBUTION
            .get(&self.status).unwrap()
            .get(&self.education).unwrap();

        let gender_value = GENDER_DISTRIBUTION
            .get(&self.status).unwrap()
            .get(&self.gender).unwrap();
    
        let mut rng = rand::thread_rng();

        let mut base_value = 
            age_value * PARAM_A * rng.gen_range(0.9..1.0) +
            education_value * PARAM_B * rng.gen_range(0.9..1.0) +
            gender_value * PARAM_C * rng.gen_range(0.05..0.1);


        // Temporarily hardcoded values for testing
        let total_nodes = 2.;
        let total_exposed = 1.;
        let total_infected = 1.;
        let total_doubtful = 0.;
        let total_recovered = 0.;
        let total_susceptible = 0.;

        match to {
            Status::Susceptible => {
                panic!("Susceptible is not a valid transition");
            }
            Status::Exposed => {
                if self.status == Status::Susceptible {
                    base_value += total_exposed / total_nodes 
                        + total_infected / total_nodes
                        - total_doubtful / total_nodes
                        - total_recovered / total_nodes;
                } else if self.status == Status::Doubtful {
                    base_value += total_exposed / total_nodes 
                        + total_infected / total_nodes
                        - total_doubtful / total_nodes
                        - total_recovered / total_nodes
                        + total_susceptible / total_nodes;
                } else {
                    panic!("Invalid transition from {} to Exposed", self.status);
                }
            }
            Status::Infected => {
                if self.status == Status::Exposed {
                    base_value += total_exposed / total_nodes 
                        + total_infected / total_nodes
                        - total_doubtful / total_nodes
                        - total_recovered / total_nodes;
                } else {
                    panic!("Invalid transition from {} to Infected", self.status);
                }
            }
            Status::Doubtful => {
                if self.status == Status::Exposed {
                    base_value +=   - total_exposed / total_nodes 
                        - total_infected / total_nodes
                        + total_doubtful / total_nodes
                        + total_recovered / total_nodes;
                } else {
                    panic!("Invalid transition from {} to Doubtful", self.status);
                }
            }
            Status::Recovered => {
                if self.status == Status::Infected {
                    base_value +=  - total_exposed / total_nodes 
                        - total_infected / total_nodes
                        + total_doubtful / total_nodes
                        + total_recovered / total_nodes
                        - total_susceptible / total_nodes;
                } else {
                    panic!("Invalid transition from {} to Recovered", self.status);
                }
            }
        }

        base_value
    }

    fn transition_state(&mut self) {
        match self.status {
            Status::Susceptible => {
                if self.compute_transition_threshold(Status::Exposed) > ALPHA {
                    self.status = Status::Exposed;
                }
            }
            Status::Exposed => {
                let threshold_i = self.compute_transition_threshold(Status::Infected);
                let threshold_d = self.compute_transition_threshold(Status::Doubtful);

                if threshold_i > BETA && threshold_d > GAMMA {
                    self.status = if (threshold_i / BETA) > (threshold_d / GAMMA) {
                        Status::Infected
                    } else {
                        Status::Doubtful
                    };
                } else if threshold_i > BETA {
                    self.status = Status::Infected;
                } else if threshold_d > GAMMA {
                    self.status = Status::Doubtful;
                }
            }
            Status::Infected => {
                if self.compute_transition_threshold(Status::Recovered) > DELTA {
                    self.status = Status::Recovered;
                }
            }
            Status::Doubtful => {
                if self.compute_transition_threshold(Status::Exposed) > THETA {
                    self.status = Status::Exposed;
                }
            }
            Status::Recovered => {}
        }
    }
}

impl Agent for Node {
    fn step(&mut self, state: &mut dyn State) {
        let state = state
            .as_any()
            .downcast_ref::<FakeNewsState>()
            .unwrap();

        self.transition_state();

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
