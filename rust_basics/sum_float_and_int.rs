fn sum_float_int() {
    let xf: f64 = 4.0;
    let xi: i16 = 5;
    let xsum = xf as i32 + xi; // here we downcast from float to integer
}

fn main() {
    sum_float_int()
}
