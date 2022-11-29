fn declare_mutable_variable() {
    let mut x: f64 = 4.0; // declaring a float64 variable that is mutable
    x = 5.0; // now this variable can be overwritten. Otherwise cargo would raise an error
    print!("Our number is....{}.", {x})
}

fn main() {
    declare_mutable_variable()
}