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
        a: [usize; n],
    }

    let vec = a.into_iter()
                .enumerate()
                .sorted_by_key(|(_i, a)| Reverse(*a))
                .collect::<Vec<(usize, usize)>>();


    for (i, _v) in vec {
        println!("{}", i + 1);
    }
}
