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
        n: usize,
        p: [usize; n],
    }

    let mut vec = vec![1_000_000_usize; n];

    for (i, mut pi) in p.into_iter().enumerate() {
        pi -= 1;
        vec[pi] = i+1;
    }

    let ans: String = vec
                    .into_iter()
                    .map(|x| x.to_string())
                    .join(" ");

    println!("{}", ans);
}
