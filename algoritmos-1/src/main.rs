use std::time::Instant;

mod bs;
mod ms;
mod qs;

fn main() {
    let now = Instant::now();

    // let v: Vec<i32> = Vec::from((1..10000).collect());
    let v: Vec<i32> = (1..10000).collect();
    let t = 1451;
    bs::binary_search(&v, t);
    let elapsed = now.elapsed().as_millis();
    println!("Time elapsed BINARY SEARCH: {}", elapsed)
}
