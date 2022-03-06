use std::collections::{HashSet, HashMap};
use proconio::input;

fn main() {
    input!{
        x: i64,
    }

    for a in -1000..1000 {
        for b in -1000..1000 {
            if a * a * a * a * a - b * b * b * b * b == x {
                println!("{} {}", a, b);
                return;
            }
        }
    }
}
