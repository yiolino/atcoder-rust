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
        mut s: Chars,
    }

    // 状態DP
    // 8行, n列のマトリクス
    let mut dp = vec![vec![0; n+1]; 8];
    for i in 0..=n {
        dp[0][i] = 1;
    }

    let atcoder = vec!['-', 'a', 't', 'c', 'o', 'd', 'e', 'r'];
    s.insert(0, '-');

    for i in 1..=7 {
        for j in 1..=n {
            if s[j] == atcoder[i] {
                dp[i][j] += dp[i-1][j-1];
                dp[i][j] %= 1_000_000_000 + 7;
            }
            dp[i][j] += dp[i][j-1];
            dp[i][j] %= 1_000_000_000 + 7;
        }
    }

    println!("{}", dp[7][n]);
}
