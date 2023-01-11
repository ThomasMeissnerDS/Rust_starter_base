fn main() {
    basic_fn();
    // adding the code block
    let full_text: String = {
        let a: String = "I like".to_string();
        let b: String = "Rust".to_string();
        format!("{a} {b}")
    };
    println!("The truth is: {full_text}.")
}

// all code between curly brackets is the function body
fn basic_fn() {
    println!("This is a basic function.")
}