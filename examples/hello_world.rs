struct Position {
    x: i32,
    y: i32,
}

struct Velocity {
    x: i32,
    y: i32,
}

mod uz_app {
    pub mod species {
        use super::super::{Position, Velocity};
        pub struct Ball {
            position: Position,
            velocity: Velocity,
        }
    }

    pub mod law {
        use crate::{Position, Velocity};

        pub trait CanMove {
            fn position_mut(&mut self) -> &mut Position;

            fn velocity(&self) -> &Velocity;

            fn movement(target: &mut impl CanMove) {
                target.position_mut().x += target.velocity().x;
                target.position_mut().y += target.velocity().y;
            }
        }
    }

    pub mod engine {
        use super::species::Ball;
        use uz::core::Engine as RawEngine;
        use uz::core::engine::Trait as EngineTrait;
        use uz::core::engine::Database as DatabaseTrait;
        use uz::core::engine::database::TableData;
        use uz::core::engine::ExecutionProcess as ExecutionProcessTrait;
        use uz::core::engine::Pointer as EnginePointer;
        use uz::core::engine::StaticVariables as StaticVariablesTrait;

        pub struct TraitConfig;

        impl EngineTrait for TraitConfig {
            type StaticVariables = StaticVariables;
            type Database = Database;
            type ExecutionProcess = ExecutionProcess;
        }

        pub type Engine = RawEngine<TraitConfig>;

        pub struct StaticVariables;
        impl StaticVariablesTrait for StaticVariables {}

        pub struct Database {
            ball_table: TableData<Ball>,
        }
        impl DatabaseTrait for Database {}

        pub struct ExecutionProcess;
        impl ExecutionProcessTrait for ExecutionProcess {
            type EngineTrait = TraitConfig;
            fn execute_all(&self, engine_pointer: EnginePointer<Self::EngineTrait>) {

            }
        }
    }
}

fn main() {}
