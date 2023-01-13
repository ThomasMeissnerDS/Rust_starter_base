fn print_alternatives() {
    let x: f64 = 4.0; // declaring a float64 variable
    println!("We can print in octal with {:o}, hexdecimal using {:X} and binary with {:b}.", x, x, x)
}

fn main() {
    print_alternatives()
}