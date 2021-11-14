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

#[fastout]
fn main() {
    input!{
        n: usize,
        k: usize,
        a: [usize; n],
    }

    // プロジェクトをP個以上作れるか？という問いに落とし込む
    let mut upper = 1e18 as usize;
    let mut lower = 0;

    while upper - lower > 1 {
        let mid = (upper + lower) / 2;

        // midがプロジェクトの個数だとする。
        let mut sum = 0;
        for ai in a.iter() {
            sum += min(*ai, mid);
        }

        if sum >= mid * k {
            lower = mid;
        } else {
            upper = mid;
        }
    }

    println!("{}", lower);
}
