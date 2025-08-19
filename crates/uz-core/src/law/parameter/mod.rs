pub mod species_dominated;

pub enum Parameter {
    Permutation(Vec<species_dominated::FormalSpeciesDeclare>),
    Combination(Vec<species_dominated::FormalSpeciesDeclare>),
}

pub struct Parameters(Vec<Parameter>);
