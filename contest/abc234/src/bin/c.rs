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
        k: usize,
    }

    let mut keta = 0;
    let mut kk = k;
    while kk > 1 {
        kk /= 2;
        keta += 1;
    }

    let mut vec = vec![];

    for i in 0..=keta {
        if (k >> i & 1) > 0 {
            vec.push('2');
        } else {
            vec.push('0');
        }
    }

    vec.reverse();
    let ans = vec.iter().collect::<String>();

    println!("{}", ans);
}
