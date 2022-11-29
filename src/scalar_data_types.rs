fn declare_unsigned_integers() {
    let x: u32 = 4; // declaring an unsigned integer (cannot be negative)
    // available as u8, u16, u32, u64
    print!("Our number is....{}.", {x})
}

fn declare_signed_integers() {
    let x: i16 = 2; // declaring a signed integer (can be negative)
    // available as i8, i16, i32, i64
    print!("Our number is....{}.", {x})
}

fn declare_float() {
    let x: f32 = 2.07; // declaring a floating point number
    // available as f32 & f64
    print!("Our number is....{}.", {x})
}

fn declare_bool() {
    let x: bool = true; // declaring a floating point number
    let not_equal: bool = x != 15;
    // available as f32 & f64
    print!("Our value is....{}.", {not_equal})
}

fn declare_string() {
    let x: str = "example string"; // declaring a floating point number
    // available as f32 & f64
    print!("Our value is....{}.", {x})
}

fn get_max_size_for_scalars() {
    print!("Our max possible number is....{}.", {std::u8::MAX});
    print!("Our max possiblenumber is....{}.", {std::i8::MAX});
    print!("Our  max possible number is....{}.", {std::f32::MAX});
}

fn main() {
    declare_unsigned_integers();
    declare_signed_integers();
    declare_float();
    declare_bool();
    get_max_size_for_scalars();
}