mod mutex;
mod bakery_algorithm;
mod dining_philosophers;

fn main() {
    // mutex::run_mutex_example();
    // bakery_algorithm::run_bakery_algorithm_example();
    dining_philosophers::default_implements_philosopers();
}