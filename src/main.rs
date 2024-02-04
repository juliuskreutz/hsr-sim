#[macro_use]
extern crate tracing;

mod character;
mod enemy;
mod engine;
mod info;
mod model;
mod target;

use engine::Engine;

fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting engine");

    Engine::new().run()
}
