/*
Modify the program below by adding suitable functions so that it compiles correctly


fn main() {

    let x = (5 + 3) * (6 + 4);

    let y = times(add_3(5), add_4(6));

    assert_eq!(x, y);

    println!("Good job!");

}
 */
fn main() {

    let x = (5 + 3) * (6 + 4);

    let y = times(add_3(5), add_4(6));

    assert_eq!(x, y);

    println!("Good job!");

}

fn times(num_1: i32, num_2: i32) -> i32 {
    num_1 * num_2
}


fn add_3(num: i32) -> i32 {
    let pre_num: i32 = 3;
    num + pre_num
}

fn add_4(num: i32) -> i32 {
    let pre_num: i32 = 4;
    num + pre_num
}
