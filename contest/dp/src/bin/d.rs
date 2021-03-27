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
        wv: [[usize; 2]; N],
    }

    let mut dp = vec![vec![0; W+1]; N+1];

    for i in 1..=N {
        for j in 1..=W {
            if j as i64 - wv[i-1][0] as i64>= 0 {
                dp[i][j] = dp[i][j].max(dp[i-1][j - wv[i-1][0]] + wv[i-1][1]);
            }

            dp[i][j] = dp[i][j].max(dp[i-1][j]);
        }
        
    }

    println!("{}", dp[N][W]);
}
