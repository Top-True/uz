use crate::species;
use utils::reflect::TypeMetadata;

pub struct FormalSpeciesDeclare {
    pub children: Vec<(species::attribute::Name, FormalSpeciesDeclare)>,
    pub input_fields: Vec<(species::attribute::Name, TypeMetadata)>,
    pub output_fields: Vec<(species::attribute::Name, TypeMetadata)>,
}
