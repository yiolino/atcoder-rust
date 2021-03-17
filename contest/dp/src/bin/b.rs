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
        K: usize,
        h: [i64; N],
    }

    let mut dp = vec![std::i64::MAX; N];

    dp[0] = 0;

    for i in 0..N {
        for j in 1..=K {
            if i + j > N - 1 {
                continue;
            }
            dp[i + j] = dp[i + j].min(dp[i] + (h[i] - h[i + j]).abs());
        }
    }


    println!("{}", dp[N - 1]);
}
