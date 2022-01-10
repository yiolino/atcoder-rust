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
        mut m: usize,
    }

    // 常にn >= m とする
    if m > n {
        std::mem::swap(&mut n, &mut m);
    }

    let ans: usize;

    if n == 1 && m == 1 {
        ans = 1;
    } else if n > 1 && m == 1 {
        ans = n - 2;
    } else {
        ans = (n - 2) * (m - 2);
    }

    println!("{}", ans);
}
