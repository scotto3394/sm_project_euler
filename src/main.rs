
extern crate project_euler;
use project_euler::solutions::*;

fn main() {
    let mut cache = DivisorCache::new();

    println!("{}", cache.get(2));
}
