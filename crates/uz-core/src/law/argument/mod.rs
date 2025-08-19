pub mod species_dominated;

pub trait Argument {}
pub struct Arguments(Vec<Box<dyn Argument>>);

pub struct Permutation;
impl Argument for Permutation {}

pub struct Combination;
impl Argument for Combination {}
