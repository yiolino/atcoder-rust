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
        M: usize,
        a: [usize; M],
    }

    let mut oks = vec![true; N+1];
    for ai in &a {
        oks[*ai] = false;
    }

    let mut dp = vec![0; N+1];
    dp[0] = 1;

    for i in 0..N {
        for j in i+1..=min(i+2, N) {
            if oks[j] {
                dp[j] += dp[i];
                dp[j] %= 1_000_000_007;
            }
        }
    }

    println!("{}", dp[N]);
}
