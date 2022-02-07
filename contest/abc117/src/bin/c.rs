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
        m: usize,
        mut x: [i64; m]
    }

    if n >= m {
        println!("0");
        return;
    }


    // コマが担当する区間を決めてやれば良い
    x.sort();
    let mut kukan = vec![];
    for i in 1..m {
        kukan.push(x[i] - x[i - 1]);
    }

    kukan.sort();

    for _ in 0..(n-1) {
        kukan.pop();
    }

    let ans: i64 = kukan.iter().sum();

    println!("{}", ans);
}
