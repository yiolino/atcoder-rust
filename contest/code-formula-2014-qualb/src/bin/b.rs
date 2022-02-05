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
        n: String,
    }

    let mut vec: Vec<char> = n.chars().collect();

    vec.reverse();

    let mut odd = 0;
    let mut even = 0;

    for (i, v) in vec.into_iter().enumerate() {
        if (i + 1) % 2 != 0 {
            odd += v.to_digit(10).unwrap();
        } else {
            even += v.to_digit(10).unwrap();
        }
    }

    println!("{} {}", even, odd);
}
