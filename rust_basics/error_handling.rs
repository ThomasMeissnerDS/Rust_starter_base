fn main() {
    let a = divide(4, 5);
    let b = divide(10, 0);

    /* Matching is good for multiple cases */
    match a {
        Ok(v) => println!("val = {}", v),
        _ => {}/* Underscores says.. for eather other case do XYZ*/
    }
    /* For single-case matches this can be reduced to:*/
    if let Ok(v) = a {
        println!("val = {}", v)
    }

    println!("a = {:?}, b = {:?}", a, b);
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Cannot divide by zero".to_string())
    }
    Ok(a / b)
}