fn main() {
    basic_fn();
    with_input_fn("Thomas", 1500);
    let result: i32 = with_output_fn(5, 100);
    println!("This is my result: {}", result);
}

// all code between curly brackets is the function body
fn basic_fn() {
    println!("This is a basic function.")
}

fn with_input_fn(name: &str, salary: i32) {
    println!("{} earns {}€.", name, salary);
}

fn with_output_fn(num1: i32, num2: i32) -> i32 {
    num1 * num2
}