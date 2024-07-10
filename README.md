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
