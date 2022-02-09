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
        x1: f64,
        y1: f64,
        mut x2: f64, 
        mut y2: f64,
        mut x3: f64,
        mut y3: f64,
    }

    // 原点移動させて、符号付き面積の公式を用いる
    x2 -= x1;
    x3 -= x1;
    y2 -= y1;
    y3 -= y1;

    let ans = (x2 * y3 - x3 * y2).abs() / 2.0;


    println!("{}", ans);
}
