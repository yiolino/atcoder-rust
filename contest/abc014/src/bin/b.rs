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
        x: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    for i in 0..n {
        if 1<<i & x > 0 {
            ans += a[i];
        }
    }

    println!("{}", ans);
}
