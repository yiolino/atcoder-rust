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
        w: usize,
        h: usize,
        x: usize,
        y: usize,
    }

    let is_multiple: usize;
    if w / 2 == x && w % 2 == 0 && h / 2 == y && h % 2 == 0 {
        is_multiple = 1;
    } else {
        is_multiple = 0;
    }

    println!("{} {}", (w * h) as f64 / 2.0, is_multiple);
}
