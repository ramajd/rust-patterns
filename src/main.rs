use std::env;

mod algorithms;
mod implementations;
mod patterns;

use crate::algorithms::sort::run_heap_sort_algorithm;
use crate::algorithms::sort::run_merge_sort_algorithm;
use crate::algorithms::sort::run_quick_sort_algorithm;
use crate::algorithms::sort::run_selection_sort;
use crate::implementations::run_adapter_logic;
use crate::implementations::run_decorator_logic;
use crate::implementations::run_observer_logic;
use crate::implementations::run_singleton_pattern;

fn main() {
    let pattern = env::args().nth(1).expect("pattern name is not specified!");

    match pattern.as_str() {
        // Patterns
        "adapter" => run_adapter_logic(),
        "decorator" => run_decorator_logic(),
        "observer" => run_observer_logic(),
        "singleton" => run_singleton_pattern(),

        // Algorithms
        "heap_sort" => run_heap_sort_algorithm(),
        "merge_sort" => run_merge_sort_algorithm(),
        "quick_sort" => run_quick_sort_algorithm(),
        "selection_sort" => run_selection_sort(),

        _ => println!("Error: Invalid Command"),
    }
}
