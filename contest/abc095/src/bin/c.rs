#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{min, max};

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

    let mut min_price = 1_000_000_000_000;

    for i in 0..=100_000 {
        min_price = min_price.min(i * 2*C + max(0, X - i) * A + max(0, Y - i) * B);
    }
    println!("{}", min_price);
}
