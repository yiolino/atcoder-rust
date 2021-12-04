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
        mut _n: i64,
        mut a: i64,
        mut b: i64,
        mut p: i64,
        mut q: i64,
        mut r: i64,
        mut s: i64,
    }

    let mut ans = vec![vec!['.'; (s-r+1) as usize]; (q-p+1) as usize];

    let x = max(p-a, r-b);
    let y = min(q-a, s-b);

    for i in x..=y {
        ans[(i+a-p) as usize][(i+b-r) as usize] = '#';
    }

    let x = max(p-a, b-s);
    let y = min(q-a, b-r);

    for i in x..=y {
        ans[(i+a-p) as usize][(b-i-r) as usize] = '#';
    }

    for i in 0..ans.len() {
        println!("{}", ans[i].iter().collect::<String>())
    }
}
