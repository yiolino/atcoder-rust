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
        R: i64,
        X: i64,
        Y: i64,
    }

    let ans: i64;
    let d = ((X*X + Y*Y) as f64).sqrt();

    let sho = (d / R as f64).ceil() as i64;
    if sho == 1 && d as i64 != R {
        ans = 2;
    } else {
        ans = sho;
    }

    println!("{}", ans);
}
