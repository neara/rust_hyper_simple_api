mod server;
pub mod views;

#[macro_use] extern crate log;

use server::start;


fn main() {
    pretty_env_logger::init();

    info!("Starting server");

    if let Err(e) = start() {
        error!("Server error: {}", e)
    }
}
