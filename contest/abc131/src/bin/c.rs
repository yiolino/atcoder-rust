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
        c: usize,
        d: usize,
    }

    let ans = f(b, c, d) - f(a - 1, c, d);

    println!("{}", ans);
}


// 区間[1, n]の個数を求める
fn f(n: usize, c: usize, d: usize) -> usize {
    let div_c = n / c;
    let div_d = n / d;

    let lcd = num::integer::lcm(c, d);
    let common = n / lcd;

    n - div_c - div_d + common
}