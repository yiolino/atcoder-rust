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
        D: u32,
        N: i64,
    }

    if N == 100 {
        let ans = 101 * 10i64.pow(D * 2);
        println!("{}", ans);
        return;
    }

    let ans = N * 10i64.pow(D * 2);

    println!("{}", ans);
}
