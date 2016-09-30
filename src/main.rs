
extern crate project_euler;
use project_euler::solutions::*;

fn main() {
    let mut cache = CollatzCache::new();
    println!("{}", longest_collatz(1_000_000_i64, &mut cache));
}
