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

#[fastout]
fn main() {
    input!{
        mut s: Chars,
        mut t: Chars,
    }

    let slen = s.len();
    let tlen = t.len();
    s.insert(0, '0');
    t.insert(0, '1');

    // dp[i][j] := 「sをi文字目、tをj文字目までみた時の最長共通部分裂の長さ」 とする
    let mut dp = vec![vec![0; t.len()]; s.len()];

    for i in 1..s.len() {
        for j in 1..t.len() {
            if s[i] == t[j] {
                dp[i][j] = dp[i.wrapping_sub(1)][j.wrapping_sub(1)] + 1;
            } else {
                dp[i][j] = max(dp[i.wrapping_sub(1)][j], dp[i][j.wrapping_sub(1)]);
            } 
        }
    }

    let mut seq = vec![];
    let mut length = dp[s.len() - 1][t.len() - 1];
    let mut i = slen;
    let mut j = tlen;

    while length > 0 {
        if s[i] == t[j] {
            seq.push(s[i]);
            length -= 1;
            i -= 1;
            j -= 1;

        } else if dp[i][j] == dp[i.wrapping_sub(1)][j] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    seq.reverse();
    let ans = seq.into_iter().collect::<String>();

    println!("{}", ans);
}
