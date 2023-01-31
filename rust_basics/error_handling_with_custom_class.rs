#[derive(Debug)]
pub enum Res <T, E> { /* This denotes "any type" */
    Thing(T),
    Error(E)
}

fn main() {
    let a = divide(4, 5);
    let b = divide(10, 0);

    /* Matching is good for multiple cases */
    match a {
        Res::Thing(v) => println!("val = {}", v),
        _ => {}/* Underscores says.. for eather other case do XYZ*/
    }
    /* For single-case matches this can be reduced to:*/
    if let Res::Thing(v) = a {
        println!("val = {}", v)
    }

    println!("a = {:?}, b = {:?}", a, b);
}

fn divide(a: i32, b: i32) -> Res<i32, String> {
    if b == 0 {
        return Res::Error("Cannot divide by zero".to_string())
    }
    Res::Thing(a / b)
}

