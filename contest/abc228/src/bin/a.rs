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
        s: usize,
        mut t: usize,
        x: usize
    }

    let mut ans = "Yes";

    if s < t {
        if x < s || x >= t {
            ans = "No";
        }
    } else {
        if x >= t && x < s {
            ans = "No";
        } 
    }

    println!("{}", ans);
}


