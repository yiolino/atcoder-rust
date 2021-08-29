#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
    input!{
        xy: String,
    }

    let split = xy.split(".");

    let vec = split.collect::<Vec<&str>>();

    let x = vec[0];
    let y = vec[1].parse().unwrap();

    if y <= 2 {
        println!("{}-", x);
    } else if 3 <= y && y <= 6 {
        println!("{}", x);
    } else {
        println!("{}+", x);
    }
}
