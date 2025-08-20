#[derive(Clone, PartialEq, Eq)]
pub struct Name(u32);

impl Name {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
}
