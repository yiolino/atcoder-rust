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
        m: usize,
    }

    let ans = if m < 100 {
        0
    } else if 100 <= m && m <= 5000 {
        m / 100 
    } else if 6000 <= m && m <= 30_000 {
        m / 1000 + 50
    } else if 35_000 <= m && m <= 70_000 {
        (m / 1000 - 30) / 5 + 80
    } else  {
        89
    };

    println!("{:02}", ans);
}
