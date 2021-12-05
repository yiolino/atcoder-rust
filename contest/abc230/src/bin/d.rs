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
        d: i64,
        mut lr: [(i64, i64); n],
    }

    lr.sort_by_key(|s| s.1); // 右端の座標でsort

    let mut ans = 0;
    let mut x = std::i64::MIN;

    for i in 0..n {
        let l  = lr[i].0;
        let r = lr[i].1;

        if x + d - 1 < l {
            x = r;
            ans += 1;
        }
    }

    println!("{}", ans);
}
