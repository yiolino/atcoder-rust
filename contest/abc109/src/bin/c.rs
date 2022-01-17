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
        n: usize,
        x: usize,
        mut x_vec: [usize; n],
    }

    // 挿入してsort
    x_vec.push(x);
    x_vec.sort();

    // 差分の最大公約数が答え
    let mut ans = x_vec[1] - x_vec[0];
    for i in 1..n {
        ans = num::integer::gcd(ans, x_vec[i + 1] - x_vec[i]);
    }

    println!("{}", ans);
}
