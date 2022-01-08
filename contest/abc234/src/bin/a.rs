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
        t: usize,
    }

    let a0 = f(t);
    let a1 = f(t) + t;
    let a2 = f(a0);
    let a3 = f(a1);

    let ans = f(a2 + a3);

    println!("{}", ans);
}

fn f(t: usize) -> usize {
    t * t + 2 * t + 3
}