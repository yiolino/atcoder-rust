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

    let mut dp = vec![vec![0; max_v + 1]; N + 1];

    for (i, (weight, value)) in wv.iter().enumerate() {
        for j in 1..=max_v {
            if (j - *weight) as i64 >= 0 {
            dp[i + 1][j] = dp[i][j].min(dp[i][j - *value] + *weight);
            }

        }
    }





    println!();
}
