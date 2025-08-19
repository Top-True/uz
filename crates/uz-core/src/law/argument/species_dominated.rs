use crate::species;
use std::ptr::NonNull;
use utils::reflect::TypeMetadata;

pub struct InputDataHandle {
    type_metadata: TypeMetadata,
    pointer: NonNull<u8>,
}

pub struct OutputDataHandle {
    type_metadata: TypeMetadata,
    pointer: NonNull<u8>,
}

pub struct ActualSpeciesData {
    pub children: Vec<(species::attribute::Name, ActualSpeciesData)>,
    pub input_fields: Vec<(species::attribute::Name, InputDataHandle)>,
    pub output_fields: Vec<(species::attribute::Name, OutputDataHandle)>,
}
