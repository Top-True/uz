use super::{Pointer as EnginePointer, Trait as EngineTrait};

pub struct Executor<EngineTraitType: EngineTrait>(
    fn(engine_pointer: EnginePointer<EngineTraitType>),
);

impl<EngineTraitType: EngineTrait> Executor<EngineTraitType> {
    pub fn new(function: fn(engine_pointer: EnginePointer<EngineTraitType>)) -> Self {
        Self(function)
    }

    pub fn execute(&self, engine_pointer: EnginePointer<EngineTraitType>) {
        self.0(engine_pointer)
    }
}
