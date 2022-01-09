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
        n: usize,
        mut s: [usize; n],
    }

    // dpで解いてみる。

    // dp[i][j] := i個までの問題を使って、合計得点をj点にできるか と定義する。
    let mut dp = vec![vec![false; 10_001]; n + 1];
    dp[0][0] = true;

    for i in 0..n {
        for j in 0..=10_000 {
            if dp[i][j] {
                // i+1が不正解
                dp[i + 1][j] = true;
                // i+1が正解
                dp[i + 1][j + s[i]] = true;
            }
        }
    }

    let mut ans = 0;
    for (j, is_ok) in dp[n].iter().enumerate() {
        if *is_ok && j % 10 != 0 {
            ans = ans.max(j);
        }
    }

    println!("{}", ans)
}
