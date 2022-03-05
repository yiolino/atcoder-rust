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

const MOD: i64 = 998244353;

fn main() {
    input!{
        n: usize
    }

    // dpで解く
    let mut dp = vec![vec![0; 9]; n];
    for i in 0..9 {
        dp[0][i] = 1;
    }

    for h in 1..n {
        for w in 0..9 {
            if w == 0 {
                dp[h][w] += dp[h - 1][w];
                dp[h][w] %= MOD;
                dp[h][w] += dp[h - 1][w + 1];
                dp[h][w] %= MOD;
            } else if w == 8 {
                dp[h][w] += dp[h - 1][w];
                dp[h][w] %= MOD;
                dp[h][w] += dp[h - 1][w - 1];
                dp[h][w] %= MOD;
            } else {
                dp[h][w] += dp[h - 1][w];
                dp[h][w] %= MOD;
                dp[h][w] += dp[h - 1][w + 1];
                dp[h][w] %= MOD;
                dp[h][w] += dp[h - 1][w - 1];
                dp[h][w] %= MOD;
            }
        }
    }

    let mut ans = 0;
    for i in 0..9 {
        ans += dp[n-1][i];
        ans %= MOD;
    }

    ans += MOD;
    ans %= MOD;

    println!("{}", ans);
}
