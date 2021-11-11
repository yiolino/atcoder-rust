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
        abc: [(usize, usize, usize); n],
    }

    // dp[i][j] := i日目にj(a ~ c)を選んだ時の総和の最大値
    let mut dp = vec![vec![0; n]; 3];

    // 0日目は先に入れとく
    dp[0][0] = abc[0].0;
    dp[1][0] = abc[0].1;
    dp[2][0] = abc[0].2;

    for i in 1..n {
        dp[0][i] = max(dp[1][i.wrapping_sub(1)] + abc[i].0, dp[2][i.wrapping_sub(1)] + abc[i].0);
        dp[1][i] = max(dp[0][i.wrapping_sub(1)] + abc[i].1, dp[2][i.wrapping_sub(1)] + abc[i].1);
        dp[2][i] = max(dp[0][i.wrapping_sub(1)] + abc[i].2, dp[1][i.wrapping_sub(1)] + abc[i].2);
    }

    let mut ans = 0;

    for i in 0..3 {
        ans = ans.max(dp[i][n-1]);
    }

    println!("{}", ans);
}
