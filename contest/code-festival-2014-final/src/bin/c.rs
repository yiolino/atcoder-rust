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
        mut n: u64,
    }

    for mut k in 10..=10000 {
        let kk = k;
        let mut a = 0;
        let mut i = 0;
        while k > 0 {
            a += (k % 10) * (kk as u64).pow(i);
            k /= 10;
            i += 1;
        }

        if a == n {
            println!("{}", kk);
            return;
        }
    }

    println!("-1");
}
