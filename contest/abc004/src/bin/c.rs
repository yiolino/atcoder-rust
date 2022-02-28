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
        mut n: usize,
    }

    let mut vec = vec![1, 2, 3, 4, 5, 6];

    n %= 30;

    for i in 0..n {
        let i_1 = i % 5;
        let i_2 = (i % 5) + 1;

        vec.swap(i_1, i_2);
    }


    println!("{}", vec.iter().join(""));
}
