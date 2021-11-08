#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{min, max};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        abc: [[i64; 3]; N],
    }

    let mut dp = vec![vec![0; 3]; N+1];

    for i in 0..3 {
        dp[1][i] = abc[0][i];
    }

    for i in 1..N {
        for j in 0..3 {
            for k in 0..3 {
                if j == k {
                    continue;
                }
                dp[i+1][j] = max(dp[i][k] + abc[i][j], dp[i+1][j]);
            }
        }
    }

    println!("{}", max(max(dp[N][0], dp[N][1]), dp[N][2]));
}
