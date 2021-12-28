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
    let n = 20;

    let mut vec = vec![0; n];

    vec[0] = 100;
    vec[1] = 100;
    vec[2] = 200;

    for i in 3..n {
        vec[i] = vec[i-1] + vec[i-2] + vec[i-3];
    }

    println!("{}", vec[n-1]);
}
