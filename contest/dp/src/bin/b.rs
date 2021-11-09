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
        k: usize,
        mut h: [i32; n],
    }

    // 1始まりにする
    h.insert(0, 0);

    // DPメモ配列
    let mut dp = vec![std::i32::MAX; n+1];
    dp[1] = 0;


    for i in 1..=n {
        for j in 1..=k {
            if i + j <= n{
                let abs = (h[i] - h[i+j]).abs();
                dp[i+j] = dp[i+j].min(dp[i] + abs);
            }
        }
    }

    println!("{}", dp[n]);
}
