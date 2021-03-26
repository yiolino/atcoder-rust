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
    }

    let mut dp = vec![std::i64::MAX; N+1];
    dp[0] = 0;

    for i in 0..=N {
        if i + 1 <= N {
            dp[i + 1] = min(dp[i+1], dp[i] + 1);
        }

        let mut six_pow: usize = 6;
        while i + six_pow <= N {
            dp[i + six_pow] = min(dp[i + six_pow], dp[i] + 1);
            six_pow *= 6;
        }

        let mut nine_pow: usize = 9;
        while i + nine_pow <= N {
            dp[i + nine_pow] = min(dp[i + nine_pow], dp[i] + 1);
            nine_pow *= 9;
        }
    }


    println!("{}", dp[N]);
}
