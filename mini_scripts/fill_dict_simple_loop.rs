use::std::collections::HashMap;

fn main() {
    let mut counter = HashMap::new();
    for i in 20..10000000 {
     counter.insert(i, true);
    };
    let size = counter.keys().len();
    println!("{:?}", size)
}