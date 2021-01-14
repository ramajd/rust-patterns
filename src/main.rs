use std::env;

mod algorithms;
mod patterns;

use crate::algorithms::sort::run_heap_sort_algorithm;
use crate::algorithms::sort::run_insertion_sort_algorithm;
use crate::algorithms::sort::run_merge_sort_algorithm;
use crate::algorithms::sort::run_quick_sort_algorithm;
use crate::algorithms::sort::run_selection_sort;
use crate::patterns::run_abstract_factory_pattern;
use crate::patterns::run_adapter_logic;
use crate::patterns::run_builder_pattern;
use crate::patterns::run_decorator_logic;
use crate::patterns::run_observer_logic;
use crate::patterns::run_singleton_pattern;

fn main() {
    let pattern = env::args().nth(1).expect("pattern name is not specified!");

    match pattern.as_str() {
        // Patterns
        "abstract_factory" => run_abstract_factory_pattern(),
        "adapter" => run_adapter_logic(),
        "builder" => run_builder_pattern(),
        "decorator" => run_decorator_logic(),
        "observer" => run_observer_logic(),
        "singleton" => run_singleton_pattern(),

        // Algorithms
        "heap_sort" => run_heap_sort_algorithm(),
        "insertion_sort" => run_insertion_sort_algorithm(),
        "merge_sort" => run_merge_sort_algorithm(),
        "quick_sort" => run_quick_sort_algorithm(),
        "selection_sort" => run_selection_sort(),

        _ => println!("Error: Invalid Command"),
    }
}
