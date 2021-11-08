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
        h: [i32; n],
    }

    let mut dp = vec![std::i32::MAX; n];

    dp[0] = 0;

    for i in 1..n {
        let abs = (h[i] - h[i.wrapping_sub(1)]).abs();
        dp[i] = dp[i].min(dp[i.wrapping_sub(1)] + abs);
        if i >= 2 {
            let abs = (h[i] - h[i.wrapping_sub(2)]).abs();
            dp[i] = dp[i].min(dp[i.wrapping_sub(2)] + abs);
        }
    }

    println!("{}", dp[n-1]);
}
