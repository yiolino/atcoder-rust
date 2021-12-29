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
        r: Chars,
    }

    let mut ans = 0.0_f64;

    for ri in r {
        match ri {
            'A' => ans += 4.0,
            'B' => ans += 3.0,
            'C' => ans += 2.0,
            'D' => ans += 1.0,
            _ => ans += 0.0,
        };
    }

    ans  /= n as f64;

    println!("{}", ans);
}
