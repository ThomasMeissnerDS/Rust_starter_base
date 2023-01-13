fn print_alternatives() {
    let x: f64 = 4.0; // declaring a float64 variable
    let y: f64 = 8.0; // declaring a float64 variable
    println!("Our numbers are....{} & {}.", {x}, {y}); // next print is on a new line
    println!("Our numbers are....{x_val} & {y_val}.", x_val=x, y_val=y); // next print is on a new line
    print!("Our number is....{}.", {x});  // next print will be right after on the same line
}

fn print_complex_types() {
    let x: f64 = 4.0; // declaring a float64 variable
    let y: i64 = 4567; // declaring an int variable
    println!("Our numbers are....{:?}.", (x, y)); // next print is on a new line
}

fn main() {
    print_alternatives()
}