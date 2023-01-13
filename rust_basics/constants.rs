fn constants() {
    // you are not allowed to use mut with constants (they are always immutable)
    const NEW_CONST: f64 = 4.0;

    println!("The value of s is {}.", NEW_CONST)
}

fn main() {
    constants()
}