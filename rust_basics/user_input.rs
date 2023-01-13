fn main() {
    // create empty string
    let mut n: String = String::new();
    std::io::stdin()
        .read_line(&mut n) // two possible results: error & ok
        .expect("Failed to read input.");

    let n: f64 = n.trim().parse().expect("Invalid input"); // trim removes white spaces, parse converts string into another type (f64 here)
    println!("{:?}", n);

}
