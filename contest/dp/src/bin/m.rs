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

const MOD: i64 = 1_000_000_000 + 7;

fn main() {
    input!{
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.insert(0, 0);

    // dp[i][j] := i番目までの子供にj個の飴を配る場合の数
    let mut dp = vec![vec![0; k+1]; n+1];
    dp[0][0] = 1_i64;
    
    for i in 1..=n {
        // ここで累積和を計算する
        let mut cum = vec![0; k+2];
        cum[0] = 1;
        for j in 1..=(k+1) {
            cum[j] = cum[j-1] + dp[i-1][j-1] % MOD;
        }
        for j in 0..=k {
            dp[i][j] = (cum[j+1]-cum[max(0, j as i64 - a[i] as i64) as usize] + MOD) % MOD;
        }
    }

    println!("{}", dp[n][k]);
}
  