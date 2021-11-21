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

#[fastout]
fn main() {
    input!{
        n: usize,
        x: usize,
        mut a: [usize; n],
    }

    a.insert(0, 0);

    let mut ans = 0;
    let mut known = vec![false; n+1];
    let mut idx = x;

    while !known[idx] {
        known[idx] = true;
        ans += 1;
        idx = a[idx];
    }

    println!("{}", ans);
}
