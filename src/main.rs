use std::env;

mod implementations;
mod patterns;

use crate::implementations::run_observer_logic;
use crate::implementations::run_singleton_pattern;

fn main() {
    let pattern = env::args().nth(1).expect("pattern name is not specified!");

    match pattern.as_str() {
        "singleton" => run_singleton_pattern(),
        "observer" => run_observer_logic(),
        _ => println!("Error: Invalid pattern"),
    }
}
