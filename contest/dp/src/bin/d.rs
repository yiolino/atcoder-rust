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

    let mut dp = vec![vec![0; w+1]; n+1];
    wv.insert(0, (0, 0));

    for i in 1..=n {
        for j in 0..=w {
            if j < wv[i].0 {
                dp[i][j] = dp[i.wrapping_sub(1)][j];
            } else {
                let tuika = dp[i.wrapping_sub(1)][j.wrapping_sub(wv[i].0)] + wv[i].1;
                let sonomama = dp[i.wrapping_sub(1)][j];
                dp[i][j] = max(tuika, sonomama);
            }
        }
    }

    let ans = dp[n].iter().max().unwrap();

    println!("{}", ans);
}
