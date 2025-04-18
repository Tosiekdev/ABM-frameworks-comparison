use std::collections::HashMap;
use once_cell::sync::Lazy;
use krabmaga::rand::prelude::*;
use krabmaga::rand::distributions::{Distribution, Standard};

use super::status::Status;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl Distribution<Gender> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        match rng.gen_range(0..=2) {
            0 => Gender::Male,
            1 => Gender::Female,
            _ => Gender::Other
        }
    }
}

pub static GENDER_DISTRIBUTION: Lazy<HashMap<Status, HashMap<Gender, f32>>> = Lazy::new(|| {

    let susceptible_gender = HashMap::from([
        (Gender::Male, 0.55),
        (Gender::Female, 0.4),
        (Gender::Other, 0.05),
    ]);

    let exposed_gender = HashMap::from([
        (Gender::Male, 0.48),
        (Gender::Female, 0.48),
        (Gender::Other, 0.04),
    ]);

    let infected_gender = HashMap::from([
        (Gender::Male, 0.45),
        (Gender::Female, 0.5),
        (Gender::Other, 0.05),
    ]);

    let doubtful_gender = HashMap::from([
        (Gender::Male, 0.5),
        (Gender::Female, 0.45),
        (Gender::Other, 0.05),
    ]);

    let recovered_gender = HashMap::from([
        (Gender::Male, 0.52),
        (Gender::Female, 0.43),
        (Gender::Other, 0.05),
    ]);

    HashMap::from([
        (Status::Susceptible, susceptible_gender),
        (Status::Exposed, exposed_gender),
        (Status::Infected, infected_gender),
        (Status::Doubtful, doubtful_gender),
        (Status::Recovered, recovered_gender),
    ])
});
