fn main() {
    /*
    A simple calculator to add up 2 floating points from command line
     */
    let args: Vec<String> = std::env::args().collect();
    let x: i32 = args[1].parse().unwrap();
    let y: i32 = args[2].parse().unwrap();
    let sum = x + y;
    println!("The sum of {} and {} is {}", x, y, sum);
}