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
        mut dcs:[(usize,usize,usize); n],
    }

    // 締め切りの順にsort
    dcs.sort_by_key(|p| p.0);

    // 動的計画法
    let max_d = dcs[n-1].0;
    let mut dp = vec![vec![0; max_d+1]; n+1];

    for i in 1..=n {
        let d = dcs[i.wrapping_sub(1)].0;
        let c = dcs[i.wrapping_sub(1)].1;
        let s = dcs[i.wrapping_sub(1)].2;

        for j in 0..=max_d {
            if j < c || j > d {
                dp[i][j] = dp[i.wrapping_sub(1)][j];
            } else {
                dp[i][j] = max(dp[i.wrapping_sub(1)][j.wrapping_sub(c)] + s, dp[i.wrapping_sub(1)][j]);
            }
        }
    }

    let mut ans = 0;
    for i in 0..=max_d {
        ans = ans.max(dp[n][i]);
    }

    println!("{}", ans);
}
