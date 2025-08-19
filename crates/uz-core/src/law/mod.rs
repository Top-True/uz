pub mod argument;
pub mod parameter;

pub trait Law {
    fn parameters(&self) -> parameter::Parameters;

    fn execute(&self, arguments: argument::Arguments);
}
