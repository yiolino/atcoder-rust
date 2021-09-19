#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
    input!{
        n: usize,
        x: usize, y: usize,
        ab: [[usize; 2]; n],
    }

    // 動的計画法で解く
    // dp[i][j][k]: 0~i番目までの弁当を買うか買わないかが決定しており、j個のたこ焼きとk個のたい焼きが手に入っている状態での最小の弁当の個数

    let mut dp = vec![vec![vec![1000; y+1]; x+1]; n+1];

    dp[0][0][0] = 0;

    for i in 1..=n {
        let ai = ab[i-1][0];
        let bi = ab[i-1][1];

        for j in 0..=x {
            for k in 0..=y {
                // i-1, j, k状態からi番目の弁当を買うか買わないかを考える。
                
                // 買う場合、i, j+ai, k+biになる。
                dp[i][min(j+ai, x)][min(k+bi, y)] = min(dp[i-1][j][k] + 1, dp[i][min(j+ai, x)][min(k+bi, y)]);
                // 買わない場合、i, j, kになる。
                dp[i][j][k] = min(dp[i][j][k], dp[i-1][j][k]);
            }
        }
    }

    let ans = if dp[n][x][y] == 1000 {
        -1
    } else {
        dp[n][x][y] as i64
    };

    println!("{}", ans);
}
