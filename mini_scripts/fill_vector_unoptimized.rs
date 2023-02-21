use std::time::Instant;
use itertools::Itertools;

fn main() {
    let now = Instant::now();

    let mut vector: Vec<i32> = vec![];
    for i in 0..20000 {
        let u = i as usize;
        if vector.contains(&i) {
            continue
        }
        else {
            vector.push(i);
        }
    };
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let size = vector.into_iter().unique().count();
    println!("{:?}", size);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}