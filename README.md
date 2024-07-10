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
