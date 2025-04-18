use std::collections::HashMap;
use once_cell::sync::Lazy;
use krabmaga::rand::prelude::*;
use krabmaga::rand::distributions::{Distribution, Standard};

use super::status::Status;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Age {
    From00to09,
    From10to19,
    From20to29,
    From30to39,
    From40to49,
    From50to59,
    From60to69,
    From70to79,
    From80toXX
}

impl Distribution<Age> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Age {
        match rng.gen_range(0..=8) {
            0 => Age::From00to09,
            1 => Age::From10to19,
            2 => Age::From20to29,
            3 => Age::From30to39,
            4 => Age::From40to49,
            5 => Age::From50to59,
            6 => Age::From60to69,
            7 => Age::From70to79,
            _ => Age::From80toXX
        }
    }
}

pub static AGE_DISTRIBUTION: Lazy<HashMap<Status, HashMap<Age, f32>>> = Lazy::new(|| {

    let susceptible_age = HashMap::from([
        (Age::From00to09, 0.),
        (Age::From10to19, 0.05),
        (Age::From20to29, 0.4),
        (Age::From30to39, 0.3),
        (Age::From40to49, 0.15),
        (Age::From50to59, 0.07),
        (Age::From60to69, 0.03),
        (Age::From70to79, 0.),
        (Age::From80toXX, 0.),
    ]);

    let exposed_age = HashMap::from([
        (Age::From00to09, 0.02),
        (Age::From10to19, 0.1),
        (Age::From20to29, 0.3),
        (Age::From30to39, 0.25),
        (Age::From40to49, 0.15),
        (Age::From50to59, 0.1),
        (Age::From60to69, 0.05),
        (Age::From70to79, 0.02),
        (Age::From80toXX, 0.01),
    ]);

    let infected_age = HashMap::from([
        (Age::From00to09, 0.01),
        (Age::From10to19, 0.26),
        (Age::From20to29, 0.41),
        (Age::From30to39, 0.2),
        (Age::From40to49, 0.07),
        (Age::From50to59, 0.03),
        (Age::From60to69, 0.01),
        (Age::From70to79, 0.01),
        (Age::From80toXX, 0.),
    ]);

    let doubtful_age = HashMap::from([
        (Age::From00to09, 0.),
        (Age::From10to19, 0.05),
        (Age::From20to29, 0.35),
        (Age::From30to39, 0.35),
        (Age::From40to49, 0.15),
        (Age::From50to59, 0.05),
        (Age::From60to69, 0.03),
        (Age::From70to79, 0.02),
        (Age::From80toXX, 0.),
    ]);

    let recovered_age = HashMap::from([
        (Age::From00to09, 0.),
        (Age::From10to19, 0.1),
        (Age::From20to29, 0.35),
        (Age::From30to39, 0.30),
        (Age::From40to49, 0.15),
        (Age::From50to59, 0.07),
        (Age::From60to69, 0.02),
        (Age::From70to79, 0.01),
        (Age::From80toXX, 0.),
    ]);

    HashMap::from([
        (Status::Susceptible, susceptible_age),
        (Status::Exposed, exposed_age),
        (Status::Infected, infected_age),
        (Status::Doubtful, doubtful_age),
        (Status::Recovered, recovered_age),
    ])
});

