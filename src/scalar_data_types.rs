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
    print!("Our number is....{}.", {x})
}

fn get_max_size_for_scalars() {
    print!("Our number is....{}.", {std::u8::MAX});
    print!("Our number is....{}.", {std::i8::MAX});
}

fn main() {
    declare_unsigned_integers();
}