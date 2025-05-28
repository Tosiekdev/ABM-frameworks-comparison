use std::collections::HashMap;
use once_cell::sync::Lazy;
use krabmaga::rand::prelude::*;
use krabmaga::rand::distributions::{Distribution, Standard};

use super::status::Status;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Education {
    Primary,
    Secondary,
    Higher,
    Vocational,
}

impl Distribution<Education> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Education {
        match rng.gen_range(0..=3) {
            0 => Education::Primary,
            1 => Education::Secondary,
            2 => Education::Higher,
            _ => Education::Vocational,
        }
    }
}

pub static EDUCATION_DISTRIBUTION: Lazy<HashMap<Status, HashMap<Education, f32>>> = Lazy::new(|| {

    let susceptible_education = HashMap::from([
        (Education::Primary, 0.02),
        (Education::Secondary, 0.18),
        (Education::Higher, 0.75),
        (Education::Vocational, 0.05),
    ]);

    let exposed_education =HashMap::from([
        (Education::Primary, 0.1),
        (Education::Secondary, 0.4),
        (Education::Higher, 0.45),
        (Education::Vocational, 0.05),
    ]);

    let infected_education = HashMap::from([
        (Education::Primary, 0.05),
        (Education::Secondary, 0.3),
        (Education::Higher, 0.6),
        (Education::Vocational, 0.05),
    ]);

    let doubtful_education = HashMap::from([
        (Education::Primary, 0.03),
        (Education::Secondary, 0.27),
        (Education::Higher, 0.65),
        (Education::Vocational, 0.05),
    ]);

    let recovered_education = HashMap::from([
        (Education::Primary, 0.04),
        (Education::Secondary, 0.35),
        (Education::Higher, 0.55),
        (Education::Vocational, 0.06),
    ]);

    HashMap::from([
        (Status::Susceptible, susceptible_education),
        (Status::Exposed, exposed_education),
        (Status::Infected, infected_education),
        (Status::Doubtful, doubtful_education),
        (Status::Recovered, recovered_education),
    ])
});
