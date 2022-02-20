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
        x: i64,
        ab: [(i64, i64); n],
    }

    let mut dp = vec![vec![false; 10001]; n + 1];

    dp[0][0] = true;

    for i in 0..n {
        for j in 0..=10000 {
            if dp[i][j] {
                if j as i64 + ab[i].0 <= 10000 {
                    dp[i + 1][j + ab[i].0 as usize] = true;
                }
                if j as i64 + ab[i].1 <= 10000 {
                    dp[i + 1][j + ab[i].1 as usize] = true;
                }
            }
        }
    } 

    if dp[n][x as usize] {
        println!("Yes")
    } else {
        println!("No");
    }
}
