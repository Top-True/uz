use utils::reflect::TypeMetadata;

pub mod attribute;

#[derive(Clone, PartialEq, Eq)]
pub struct ID(u128);

impl ID {
    pub const fn new(value: u128) -> Self {
        Self(value)
    }
}

pub struct MetaInfo {
    id: ID,
    fields: Vec<(attribute::Name, TypeMetadata)>,
    children: Vec<(attribute::Name, ID)>,
}

impl MetaInfo {
    pub fn new(
        id: ID,
        fields: Vec<(attribute::Name, TypeMetadata)>,
        children: Vec<(attribute::Name, ID)>,
    ) -> Self {
        Self {
            id,
            fields,
            children,
        }
    }
}
