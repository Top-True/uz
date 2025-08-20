use std::thread;
use std::time::Duration;
use uz::prelude::*;

struct Position {
    x: i32,
    y: i32,
}

struct Velocity {
    x: i32,
    y: i32,
}

species!(Ball <> {
    position: Position,
    velocity: Velocity,
});

fn main() {
    let app_handle = AppBuilder::new().add_species(Ball).build().unwrap();
    let join_handle = {
        let block_runner = app_handle.block_run_handle().unwrap();
        thread::spawn(move || {
            block_runner.run();
        })
    };
    let app_handle = pollster::block_on(app_handle.new_active_handle()).unwrap();
    thread::sleep(Duration::from_secs(10));
    app_handle.stop();
    join_handle.join().unwrap();
}
