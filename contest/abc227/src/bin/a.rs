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

#[fastout]
fn main() {
    input!{
        n: i32,
        k: i32,
        a: i32,
    }

    let mut vec = vec![];
    vec.push(0);

    for _ in 0..1000 {
        for i in 1..=n {
            vec.push(i);
        }
    }

    let mut cnt = k;
    let mut idx = a;

    while cnt > 0 {
        idx += 1;
        cnt -= 1;
    }

    println!("{}", vec[(idx-1) as usize]);

}
