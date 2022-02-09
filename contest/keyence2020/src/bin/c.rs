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
        k: usize,
        s: usize,   
    }

    let mut vec = vec![];

    for _ in 0..k {
        vec.push(s);
    }

    for _ in 0..n-k {
        if s != 1_000_000_000 {
            vec.push(s+1);
        } else {
            vec.push(1);
        }
        
    }

    let ans: String = vec.iter().join(" ");

    println!("{}", ans);
}
