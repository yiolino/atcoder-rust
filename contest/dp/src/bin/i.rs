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

#[fastout]
fn main() {
    input!{
        n: usize,
        mut p: [f64; n],
    }

    p.insert(0, 0.0);

    // dp[i][j] := i枚のコインを投げたときに、j枚の表が出ている確率 と定義する。
    let mut dp = vec![vec![0.0; n+1]; n+1];
    dp[1][1] = p[1];
    dp[1][0] = 1.0 - p[1];
    for i in 2..=n {
        dp[i][0] = dp[i.wrapping_sub(1)][0] * (1.0-p[i]);
    }


    for i in 2..=n {
        for j in 1..=i {
            let pi = p[i];
            dp[i][j] = dp[i.wrapping_sub(1)][j.wrapping_sub(1)] * pi + dp[i.wrapping_sub(1)][j] * (1.0-pi);
        }
    }

    let mut ans = 0.0;
    for i in (n/2+1)..=n {
        ans += dp[n][i];
    }

    println!("{}", ans);
}
