#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
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
        xa: f64,
        ya: f64,
        xb: f64,
        yb: f64,
        t: f64,
        v: f64,
        n: usize,
    }

    let total_dist = t * v;
    let mut ans = "NO";

    for _ in 0..n {
        input! {x: f64, y: f64};

        let a = ((x - xa) * (x - xa) + (y - ya) * (y - ya)).sqrt();
        let b = ((x - xb) * (x - xb) + (y - yb) * (y - yb)).sqrt();

        if total_dist >= a + b {
            ans = "YES";
        }
    }

    println!("{}", ans);
}
