fn shadow_value() {
    let s: f64 = 4.0;
    let s: f64 = 4.0*4.0; // we could even use a string here
    println!("The value of s is {}.", s)
}

fn shadow_value_scope() {
    let mut s: i32 = 40;
    { // new scope here as new code segment
        let r: i32 = 60; // without let keyword here we would overwrite r from outer scope
        println!("The value of r inside inner scope is {}.", r)
    }
    println!("The value of r in outer scope is {}.", r)
}

fn main() {
    shadow_value_scope()
}