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
    }

    // n+1の長さの丸太から小さいほうから切り出せるかを二分探索
    // つまり、n+1を買うことで、小さい方から何本分賄えるかということ

    let mut lower = 0;
    let mut upper = 2_000_000_000;

    while upper - lower > 1 {
        let mid = (upper + lower) / 2;

        if mid * (mid + 1) / 2 <= n + 1 {
            lower = mid;
        } else {
            upper = mid;
        }
    }

    println!("{}", n + 1 - lower);
}
