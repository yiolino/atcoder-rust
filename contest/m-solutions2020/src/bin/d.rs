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

    let mut current = 1000;
    for i in 0..n-1 {
        let mut stock = 0;
        if a[i + 1] > a[i] {
            stock = current / a[i];
        }
        current += stock * (a[i + 1] - a[i]);
    }

    println!("{}", current);
}
