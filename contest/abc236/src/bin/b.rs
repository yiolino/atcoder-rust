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
        a: [usize; 4 * n - 1],
    }

    let mut map = HashMap::new();

    for ai in a {
        *map.entry(ai).or_insert(0) += 1;
    }

    for (k, v) in map {
        if v == 3 {
            println!("{}", k);
            return;
        }
    }
}
