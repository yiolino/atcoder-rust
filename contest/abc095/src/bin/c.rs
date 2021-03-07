#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        A: i64,
        B: i64,
        C: i64,
        X: i64,
        Y: i64,
    }

    let mut min_price = 1000_000_000_000;
    for i in 0..=100_000 {
        let cand = i * 2 * C + std::cmp::max(0, X - i) * A + std::cmp::max(0, Y - i) * B;
        min_price = std::cmp::min(min_price, cand);
    }
    
    println!("{}", min_price);
}
