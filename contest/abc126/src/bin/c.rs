
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
    }
    
    let mut ans = 0.0;
    for mut i in 1..=n {
        let mut q = 1.0 / n as f64;
        while i < k {
            i *= 2;
            q /= 2.0;            
        }
        ans += q;
    }

    println!("{}", ans);
}
