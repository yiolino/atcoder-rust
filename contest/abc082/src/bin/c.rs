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

    let mut map = HashMap::new();

    for ai in a {
        *map.entry(ai).or_insert(0) += 1_usize;
    }

    let mut ans = 0;

    for (k,v)in map {
        if k > v {
            ans += v;
        } else if k < v {
            ans += v - k;
        }
    }

    println!("{}", ans);
}
