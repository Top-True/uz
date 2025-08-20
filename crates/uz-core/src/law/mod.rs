pub mod argument;
pub mod parameter;

pub struct LawDefinition {
    pub(crate) parameters: parameter::Parameters,
    pub(crate) execute_func: fn(arguments: argument::Arguments),
}
