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
        h: usize,
        w: usize,
        hs: [Chars; h],
    }

    let mut dp = vec![vec![0; w]; h];

    for i in 0..h {
        if hs[i][0] == '#' {
            break;
        }
        dp[i][0] = 1;
    }
    for i in 0..w {
        if hs[0][i] == '#' {
            break;
        }
        dp[0][i] = 1;
    }

    
    for i in 1..h {
        for j in 1..w {
            if hs[i][j] != '#' {
                dp[i][j] = dp[i.wrapping_sub(1)][j] + dp[i][j.wrapping_sub(1)];
                dp[i][j] %= 1_000_000_000 + 7;
            }
        }
    }

    println!("{}", dp[h.wrapping_sub(1)][w.wrapping_sub(1)]);
}
