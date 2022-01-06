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
        _n: usize,
        c: Chars,
    }

    let mut cnt_r = 0;
    for ci in &c {
        if *ci == 'R' {
            cnt_r += 1;
        }
    }

    let mut cnr_w = 0;
    for i in 0..cnt_r {
        if c[i] == 'W' {
            cnr_w += 1;
        }
    }

    println!("{}", cnr_w);
}
