use super::executor::Executor;
use super::{Pointer as EnginePointer, Trait as EngineTrait};

pub trait ExecutionProcess {
    type EngineTrait: EngineTrait;
    fn execute_all(&self, engine_pointer: EnginePointer<Self::EngineTrait>);
}

pub struct ExecutorWrapper<ExecutionProcessType: ExecutionProcess> {
    pub is_enable: bool,
    pub executor: Executor<ExecutionProcessType::EngineTrait>,
}

impl<ExecutionProcessType: ExecutionProcess> ExecutorWrapper<ExecutionProcessType> {
    pub fn try_execute(&self, engine_pointer: EnginePointer<ExecutionProcessType::EngineTrait>) {
        if self.is_enable {
            self.executor.execute(engine_pointer);
        }
    }
}
