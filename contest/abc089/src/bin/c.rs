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

    let mut vec = vec![0; 5];  // MARCHの数をカウントする用

    for _ in 0..n {
        input! {s: Chars};
        match s[0] {
            'M' => vec[0] += 1,
            'A' => vec[1] += 1,
            'R' => vec[2] += 1,
            'C' => vec[3] += 1,
            'H' => vec[4] += 1,
            _ => (),
        }
    }

    let mut ans = 0_usize;
    let comb:Vec<Vec<usize>> = (0..5).combinations(3).collect();
    for c in comb {
        ans += c.iter().fold(1, |acc, i| acc * vec[*i]);
    }

    println!("{}", ans);
}
