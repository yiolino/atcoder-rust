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
        m: usize,
        ab: [(i64, i64); n],
        cd: [(i64, i64); m],
    }

    for (a, b) in ab {
        let mut min_mhtn = std::i64::MAX;
        let mut check_idx = 0;
        for i in (0..m).rev() {
            let c = cd[i].0;
            let d = cd[i].1;
            let mhtn = (a-c).abs() + (b-d).abs();

            if mhtn <= min_mhtn {
                min_mhtn = mhtn;
                check_idx = i+1;
            }
        }

        println!("{}", check_idx);
    }
}
