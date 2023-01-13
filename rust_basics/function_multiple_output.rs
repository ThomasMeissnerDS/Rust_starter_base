fn main() {
    let (by_two, by_three, by_four) = multi_output((5));
}

fn multi_output(num: i32) -> (i32, i32, i32) {
    (num*2, num*3, num*4)
}
