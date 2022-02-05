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
    }

    let mut map = HashMap::new();

    for _ in 0..n {
        input! {mut s: Chars}
        s.sort();

        *map.entry(s).or_insert(0) += 1;
    }

    let mut ans: usize = 0;
    for (_, v) in map {
        if v > 1 {
            ans += fact(v-1);
        }
    }

    println!("{}", ans);
}


fn fact(n: usize) -> usize {
    if n == 0 {
        0
    } else {
        fact(n - 1) + n
    }
}