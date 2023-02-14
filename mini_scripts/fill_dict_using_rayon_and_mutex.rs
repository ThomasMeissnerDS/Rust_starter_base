extern crate rayon;

use::std::collections::HashMap;
use std::sync::Mutex;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let mut counter = HashMap::new();
    let counter = Mutex::new(counter);
    (20..10000000).into_par_iter().for_each(|i| {
     counter.lock().unwrap().insert(i, true);
    });
    let size = counter.lock().unwrap().keys().len();
    println!("{:?}", size);

    let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
}