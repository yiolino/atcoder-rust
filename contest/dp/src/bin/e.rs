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
        w: usize,
        mut wv: [(usize, usize); n],
    }

    // n, vに関するdpマトリックスを作成する。
    let mut dp = vec![vec![w+1; 1e5 as usize + 1]; n+1];
    for i in 0..=n {
        dp[i][0] = 0;
    }

    wv.insert(0, (0, 0));


    for i in 1..=n {
        for j in 0..=(1e5 as usize) {
            if j >= wv[i].1 {
                let w1 = dp[i.wrapping_sub(1)][j];
                let w2 = dp[i.wrapping_sub(1)][j.wrapping_sub(wv[i].1)];
                dp[i][j] = min(w1, w2 + wv[i].0);

            } else {
                dp[i][j] = dp[i.wrapping_sub(1)][j];
            }
        }
    }

    let mut ans = 0;
    for j in (0..=(1e5 as usize)).rev() {
        if dp[n][j] <= w {
            ans = j;
            break;
        }
    }

    println!("{}", ans);
}
