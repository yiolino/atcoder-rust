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
        N: i64,
        K: i64,
    }

    let mut ans = 0;
    for i in 1..=N {
        for j in 1..=K {
            ans += 100 * i + j;
        }
    }

    println!("{}", ans);
}
