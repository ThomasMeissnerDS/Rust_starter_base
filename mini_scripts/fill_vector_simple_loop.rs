use std::time::Instant;
use itertools::Itertools;

fn main() {
    let now = Instant::now();

    let mut vector: Vec<i32> = vec![0; 1];
    let mut counter: i32 = 1;
    for i in 0..10000000 {
        let u = i as usize;
        if i < counter {
            vector[u] = i;
        }
        else {
            vector.push(i);
            counter += 1;
        }
    };
    println!("{}", counter);
    let size = vector.into_iter().unique().count();
    println!("{:?}", size);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}