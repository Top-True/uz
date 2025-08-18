use std::any::TypeId;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeMetadata {
    pub id: TypeId,
    pub size: usize,
    pub align: usize,
}
