#[allow(dead_code)]
pub fn single_line_example() {
    // define a closure and store it in a variable
    let add_one = |x: i32| x + 1;

    // call closure multiple times
    let result = add_one(2) * add_one(4);

    println!("Result = {}", result);
}
