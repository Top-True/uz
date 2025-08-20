use crate::species;
use std::collections::LinkedList;

pub mod database;
pub mod executor;
pub mod plugin;
pub mod static_variables;

pub(crate) struct Application {
    database: database::Database,
}

pub struct Builder {
    added_plugins: LinkedList<()>,
    added_species: LinkedList<species::MetaInfo>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            added_plugins: LinkedList::new(),
            added_species: LinkedList::new(),
        }
    }

    pub fn add_species(&mut self, species: species::MetaInfo) -> Self {
        todo!()
    }

    pub fn build(self) -> Result<Handle, ()> {
        todo!()
    }
}

pub struct Handle;

impl Handle {
    pub fn block_run_handle(&self) -> Result<BlockRunHandle, ()> {
        todo!()
    }

    pub async fn new_active_handle(self) -> Result<ActiveHandle, ()> {
        todo!()
    }
}

#[derive(Clone)]
pub struct ActiveHandle;

impl ActiveHandle {
    pub fn stop(self) {
        todo!()
    }
}

pub struct BlockRunHandle;

impl BlockRunHandle {
    pub fn run(self) {
        todo!()
    }
}
