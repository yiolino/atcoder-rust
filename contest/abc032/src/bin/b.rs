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
        s: Chars,
        k: usize,
    }

    let n = s.len();

    if k > n {
        println!("0");
        return;
    }

    let mut set = HashSet::new();

    for i in 0..=n - k {
        set.insert(&s[i..k + i]);
    }

    println!("{}", set.len());
}
