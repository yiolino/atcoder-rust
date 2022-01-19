#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input!{
        s: Chars,
    }

    let waru = 1000_000_000 + 7;
    let mut dp = vec![vec![0; s.len()]; 8];

    if s[0] == 'c' {
        dp[0][0] = 1;
    }

    for i in 1..s.len() {
        if s[i] == 'c' {
            dp[0][i] = dp[0][i-1] + 1;
            dp[0][i] %= waru;
        } else {
            dp[0][i] = dp[0][i - 1];
            dp[0][i] %= waru;
        }
    }

    let chokudai = "chokudai".to_string().chars().collect::<Vec<char>>();
    for i in 1..8 {
        for j in i..s.len() {
            dp[i][j] += dp[i][j - 1];
            dp[i][j] %= waru;

            if s[j] == chokudai[i] {
                dp[i][j] += dp[i - 1][j - 1];
                dp[i][j] %= waru;
            }
        }
    }

    println!("{}", dp[7][s.len() - 1]);
}
