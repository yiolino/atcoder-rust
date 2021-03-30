#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: i64,
        A: i64,
        B: i64,
    }

    if (B - A) % 2 == 0 {
        let ans = (B - A) / 2;
        println!("{}", ans);
    } else {
        let ans = (B - A - 1) / 2 + min(N - B, A - 1) + 1;
        println!("{}", ans);
    }
}
