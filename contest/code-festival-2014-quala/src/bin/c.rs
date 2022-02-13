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
        a: usize,
        b: usize,
    }

    // 1 ~ b年までのうるう年の数
    let mut uru_b = b / 4;
    uru_b -= b / 100;
    uru_b += b / 400;

    // 1 ~ a-1 年までのうるう年の数
    let mut uru_a = (a - 1) / 4;
    uru_a -= (a - 1) / 100;
    uru_a += (a - 1) / 400;

    println!("{}", uru_b - uru_a);
}
