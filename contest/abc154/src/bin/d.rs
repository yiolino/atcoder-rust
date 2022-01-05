#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::vec;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input!{
        n: usize,
        k: usize,
        p: [f64; n],
    }

    // 期待値は (p + 1) / 2
    let mut vec_e = vec![-1.0; n];
    for (i,pi) in p.iter().enumerate() {
        vec_e[i] = (*pi + 1.0) / 2.0;
    }

    // 積和
    let mut cum = vec![0.0; n + 1];
    for i in 1..=n {
        cum[i] = cum[i - 1] + vec_e[i-1];
    }

    let mut ans: f64 = 0.0;

    for i in 0..=n-k {
        ans = ans.max(cum[k + i] - cum[i]);
    }

    println!("{}", ans);
}
