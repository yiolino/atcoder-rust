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

    let mut ans = vec![0; 6];

    for _ in 0..n {
        input! {max: f32, min: f32};

        if max >= 35.0 {
            ans[0] += 1;
        } else if max >= 30.0 && max < 35.0 {
            ans[1] += 1;
        } else if max >= 25.0 && max < 30.0 {
            ans[2] += 1;
        } else if max < 0.0 {
            ans[5] += 1;
        }

        if min >= 25.0 {
            ans[3] += 1;
        } else if min < 0.0 && max >= 0.0 {
            ans[4] += 1;
        }
    }


    println!("{}", ans.into_iter().join(" "));
}
