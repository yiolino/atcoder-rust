#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::vec;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use superslice::Ext;


fn main() {
    input!{
        n: usize,
        a: [[usize; n];n],
    }

    // dpをどう持つか？
    // dp[S] := 左からi人めの男性と集合Sに属する女性とでペアを作るときの場合の数。
    // Sはbitとして集合を表す。
    let mut dp = vec![0; 1 << n];
    dp[0] = 1;
    let mut bitcount = vec![0; 1 << n];

    for s in 1..(1 << n) {
        bitcount[s] = bitcount[s/2] + s%2;
        for l in 0..n {
            // l ... l番目の女性
            if (s >> l & 1) > 0 && a[bitcount[s]-1][l] == 1 {
                dp[s] += dp[s - (1 << l)];
                dp[s] %= 1_000_000_000 + 7;
            }
        }
    }

    println!("{}", dp[(1<<n) - 1]);
}
