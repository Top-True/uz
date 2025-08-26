pub mod database;
pub mod execution_process;
pub mod executor;
pub mod static_variables;

use std::thread::JoinHandle;
pub use database::Database;
pub use execution_process::ExecutionProcess;
pub use executor::Executor;
pub use static_variables::StaticVariables;

pub type TickID = u64;

pub trait Trait {
    type StaticVariables: StaticVariables;
    type Database: Database;
    type ExecutionProcess: ExecutionProcess;
}

pub struct Engine<EngineTrait: Trait> {
    pub current_tick_id: TickID,
    pub static_variables: EngineTrait::StaticVariables,
    pub database: EngineTrait::Database,
    pub execution_process: EngineTrait::ExecutionProcess,
}

pub struct Handle<EngineTrait: Trait> {
    pointer: *mut EngineTrait,
}

impl<EngineTrait: Trait> Handle<EngineTrait> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn tick_step(&mut self) {
        todo!()
    }
}

#[derive(Clone)]
pub struct Pointer<EngineTrait: Trait>(*mut Engine<EngineTrait>);


