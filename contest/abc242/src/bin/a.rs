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
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    if x <= a {
        println!("1");
        return;
    } else if x > b {
        println!("0");
        return;
    }


    let bunbo = b - a;

    println!("{}", c as f64 / bunbo as f64);
}
