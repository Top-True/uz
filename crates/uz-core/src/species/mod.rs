use utils::reflect::TypeMetadata;

pub mod attribute;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ID(pub u128);

pub struct MetaInfo {
    pub id: ID,
    pub children: Vec<(attribute::Name, ID)>,
    pub fields: Vec<(attribute::Name, TypeMetadata)>,
}
