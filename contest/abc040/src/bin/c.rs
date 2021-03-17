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
        N: usize,
        a: [i64; N],
    }

    let mut dp = vec![std::i64::MAX; N];

    dp[0] = 0;

    for i in 0..N-1 {
        dp[i + 1] = dp[i + 1].min(dp[i] + (a[i] - a[i + 1]).abs());
        if i + 2 >= N {
            break;
        }
        dp[i + 2] = dp[i + 2].min(dp[i] + (a[i] - a[i + 2]).abs());
    }

    println!("{}", dp[N - 1]);
}
