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
        a: usize,
        b: usize,
        c: usize,
    }

    let mut cnt = 1;

    while cnt * c <= 1000 {
        if cnt * c >= a && cnt * c <= b {
            println!("{}", cnt * c);
            return;
        }

        cnt += 1;
    }

    println!("-1");
}
