#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        A: i64,
        B: i64,
        W: i64,
    }

    let mut max = 0;
    let mut min = std::i64::MAX;

    for n in 0..=1000_000 {
        if A * n <= 1000 * W && 1000 * W <= B * n {
            max = max.max(n);
            min = min.min(n);
        }
    }

    if max != 0 {
        println!("{} {}", min, max);
    } else {
        println!("UNSATISFIABLE");
    }
}
