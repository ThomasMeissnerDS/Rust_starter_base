fn main() {
    println!("Hello, world!");
    let a: u32; //u = unsigned, so can't be negative
    let b: i32; //i = integer that can also be negative: https://doc.rust-lang.org/book/ch03-02-data-types.html
    // let erroru: u32 = -1; --> this would fail..no except statements are allowed!
    let tup: (i32, f64, u8) = (500, 6.0, 1); // define a tuple (i32 = default)
    let (tupa, tupb, tupc) = tup; // unpack tuple so components can be printed
    println!("{} {} {}", tupa, tupb, tupc);
    println!("{} {} {}", tup.0, tup.1, tup.2); // you can access tuples with zero-indexing
    let array_s: [i32; 3] = [1, 2, 3]; // in arrays you have to define number of elements and dtype at definition time (i32 = default)
    let a: [u32; 3] = [1, 2, 3];
    let a = [-3; 5]; // get an array with three elements each being
    // build a cli tool that find the square of a number
}