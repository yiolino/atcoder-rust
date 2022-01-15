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
        mut a: usize,
        k: usize,
    }

    let nicho: usize = 2_000_000_000_000;
    let ans: usize;
    // kの値で場合分け
    if k == 0 {
        ans = nicho - a;
    } else {
        let mut cnt = 0;
        while a < nicho {
            a = a + 1 + k * a;
            cnt += 1;
        }

        ans = cnt;
    }

    println!("{}", ans);
}
