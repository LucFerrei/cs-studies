use std::time::Instant;

use crate::qs::quicksort;

mod bs;
mod ms;
mod qs;

fn main() {
    let now = Instant::now();

    let v: Vec<i32> = (1..10000).collect();
    let t = 1451;
    bs::binary_search(&v, t);
    let elapsed = now.elapsed().as_millis();
    println!("Time elapsed BINARY SEARCH: {} ms", elapsed);

    let now = Instant::now();
    let v: Vec<i32> = (1..10000).collect();
    quicksort(v);
    let elapsed = now.elapsed().as_millis();
    println!("Time elapsed QUICKSORT: {} ms", elapsed);
}
