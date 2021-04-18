#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        Q: i64,
        H: i64,
        S: i64,
        D: i64,
        N: i64,
    }

    let h = min(Q * 2, H);
    let s = min(h * 2, S);

    let ans:i64;
    if 2 * s <= D {
        ans = N * s;
    } else {
        ans = (N / 2 * D) + (N % 2 * s);
    }

   println!("{}", ans);
}
