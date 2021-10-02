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

#[fastout]
fn main() {
    input!{
        k: usize,
        mut a: Bytes,
        mut b: Bytes,
    }

    a.reverse();
    b.reverse();

    let aa:usize = a.into_iter()
            .enumerate()
            .map(|(i, x)| k.pow(i as u32) * (x - 48) as usize)
            .sum();
    let bb: usize = b.into_iter()
            .enumerate()
            .map(|(i, x)| k.pow(i as u32) * (x - 48) as usize)
            .sum();

    println!("{}", aa * bb);
}
