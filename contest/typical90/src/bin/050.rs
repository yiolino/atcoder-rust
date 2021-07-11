#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        n: usize,
        l: usize,
    }

    let mut dp = vec![0; n+1];

    dp[0] = 1;

    for i in 1..=n {
        if i < l {
            dp[i] = dp[i - 1];
        } else {
            dp[i] = dp[i - 1] + dp[i - l];
            dp[i] %= 1_000_000_000 + 7;
        }
    }

    println!("{}", dp[n]);
}
