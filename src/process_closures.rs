// Define a vec of functions which take a generic `F` argument
// bounded by `Fn(i32) -> i32`, call those functions in sequence
fn process<F>(start_value: i32, closure_pipeline: Vec<F>) -> i32
where
    F: Fn(i32) -> i32,
{
    let mut total_value = start_value;

    // Process the provided functions (aka closures)
    for closure_to_process in closure_pipeline {
        total_value = closure_to_process(total_value)
    }

    total_value
}

#[allow(dead_code)]
pub fn process_closures_example() {
    // Define closures satisfying the `Fn(i32) -> i32` bound
    let add_three = |x: i32| x + 3;
    let times_two = |x: i32| 2 * x;
    let minus_one = |x: i32| x - 1;

    // Define start value and sequence of closures to execute
    let result = process(0, vec![add_three, times_two, minus_one]);

    println!("Result = {}", result);
}
