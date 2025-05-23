use krabmaga::rand::prelude::*;
use krabmaga::rand::distributions::{Distribution, Standard};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Susceptible,
    Exposed,
    Infected,
    Doubtful,
    Recovered,
}

impl Distribution<Status> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Status {
        match rng.gen_range(0..=4) {
            0 => Status::Susceptible,
            1 => Status::Exposed,
            2 => Status::Infected,
            3 => Status::Doubtful,
            _ => Status::Recovered,
        }
    }
}
