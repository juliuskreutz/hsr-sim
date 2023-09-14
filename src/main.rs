#[macro_use]
extern crate log;

mod character;
mod enemy;
mod engine;
mod listener;
mod model;
mod target;

use engine::Engine;

fn main() {
    env_logger::init();

    info!("Starting!");

    Engine::new().run();
}
