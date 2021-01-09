use std::env;

mod algorithms;
mod implementations;
mod patterns;

use crate::algorithms::sort::run_quick_sort_algorithm;
use crate::implementations::run_adapter_logic;
use crate::implementations::run_observer_logic;
use crate::implementations::run_singleton_pattern;

fn main() {
    let pattern = env::args().nth(1).expect("pattern name is not specified!");

    match pattern.as_str() {
        // Patterns
        "adapter" => run_adapter_logic(),
        "observer" => run_observer_logic(),
        "singleton" => run_singleton_pattern(),

        // Algorithms
        "quick_sort" => run_quick_sort_algorithm(),

        _ => println!("Error: Invalid Command"),
    }
}
