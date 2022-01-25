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
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    let mut set = HashSet::new();

    for ti in t {
        set.insert(ti);
    }

    for si in s {
        if set.contains(&si) {
            println!("Yes");
        } else {
            println!("No");
        }
    }

}
