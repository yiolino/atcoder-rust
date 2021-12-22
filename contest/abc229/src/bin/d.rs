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
        s: Chars,
        k: usize,
    }

    // s[l, r)に.が何個あるかを求めるため、累積和のvecを作っておく
    // cum[i] := i番目までの . の数
    let mut cum = vec![0; 2_000_000];
    for i in 1..=s.len() {
        if s[i-1] == '.' {
            cum[i] = cum[i-1] + 1;
        } else {
            cum[i] = cum[i-1]
        }
    }

    let mut r = 0;
    let mut ans = 0;
    // leftを1つずつずらしていく
    for l in 0..s.len() {
        // どこまでずらせるか
        while r < s.len() && cum[r+1] - cum[l] <= k {
            r += 1;
        }

        ans = ans.max(r - l);
    }

    println!("{}", ans)
}
