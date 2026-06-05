
#![warn(clippy::pedantic)]
use utilities::log::*;
fn main() {
    env_logger::init();
    let l =Logger;
    l.log(Level::Warn, "Hello, world!");
    println!("Init: introduction_to_algorithms");
}
