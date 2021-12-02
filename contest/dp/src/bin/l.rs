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
        a: [i64; n],
    }

    // dp[l][r] := 区間[l, r)が残っているとき、双方がその後最善を尽くした時のスコア
    // 先行、後攻で分けて考えたいところだが、%2の余りでどちらが引くか自動的にわかる
    let mut dp = vec![vec![0; n+1]; n+1];

    for len in 1..=n {
        for l in 0..=(n-len) {
            let r = l + len;
            if (n-len) % 2 == 0 {
                let v1 = dp[l+1][r] + a[l];
                let v2 = dp[l][r.wrapping_sub(1)] + a[r.wrapping_sub(1)];
                dp[l][r] = max(v1, v2);

            } else {
                let v1 = dp[l+1][r] - a[l];
                let v2 = dp[l][r.wrapping_sub(1)] - a[r.wrapping_sub(1)];
                dp[l][r] = min(v1, v2);
            }
        }
    }


    println!("{}", dp[0][n]);
}
