# Rust-Closure-Examples
A collection of rust applications using the closure feature.

## Examples

### Single line example
A very basic example of a closure that accepts an integer, adds +1 and returns the result.
The closure is then sotred in a variable and used multiple times.

```rust
pub fn single_line_example() {
    // define a closure and store it in a variable
    let add_one = |x: i32| x + 1;

    // call closure multiple times
    let result = add_one(2) * add_one(4);

    println!("Result = {}", result);
}
```

### Multiple line example
A very basic example of a closure that does some more calculations then the previous, mainly to show the syntax for reference.

```rust
pub fn multi_line_example() {
    // define a multi-line closure
    let squared_sum = |x: i32, y: i32| -> i32 {
        // find the sum of two parameters
        let sum: i32 = x + y;

        // find the squared value of the sum
        return sum * sum;
    };

    // call the closure
    let result = squared_sum(5, 3);

    println!("Result = {}", result);
}
```

### Closure pipeline
An example of how to provide a collection of closures that satisfy a defined signature 
and generically processing them in sequence

```rust
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
```