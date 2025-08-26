use crate::engine;
use std::time::Duration;

pub trait Trait {
    type EngineTrait: engine::Trait;
}

pub struct Application<ApplicationTrait: Trait> {
    engine_handle: engine::Handle<ApplicationTrait::EngineTrait>,
    engine_tick_interval: Duration,
}
