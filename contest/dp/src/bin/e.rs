#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        W: usize,
        wv: [(usize, usize); N],
    }

    let max_v: i64 = wv.iter()
                    .map(|&x| x.1 as i64)
                    .sum();
    let max_v = max_v as usize;

    let mut dp = vec![vec![10_000_000_000; max_v + 1]; N + 1];
    dp[0][0] = 0;

    for (i, (weight, value)) in wv.iter().enumerate() {
        for j in 0..max_v+1 {
            if j as i64 - *value as i64 >= 0 {
            dp[i + 1][j] = dp[i+1][j].min(dp[i][j - *value] + *weight);
            }
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j])
        }
    }

    let mut ans = 0;
    for j in 0..max_v+1 {
        if dp[N][j] <= W {
            ans = j;
        }
    }

    println!("{}", ans);
}
