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
        mut a: [i64; n],
    }

    a.insert(0, 0);
    a.push(0);

    let sum: i64 = a.windows(2)
                    .map(|x| (x[0] - x[1]).abs())
                    .sum();

    for i in 1..=n {
        let ans = sum - (a[i] - a[i-1]).abs() - (a[i+1] - a[i]).abs() + (a[i+1] - a[i-1]).abs();
        println!("{}", ans);
    }
}
