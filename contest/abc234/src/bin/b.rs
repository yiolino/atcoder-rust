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
        xy: [[i64; 2]; n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            let dx = (xy[i][0] - xy[j][0]) * (xy[i][0] - xy[j][0]);
            let dy = (xy[i][1] - xy[j][1]) * (xy[i][1] - xy[j][1]);

            ans = ans.max(dx + dy);
        }
    }

    println!("{}", (ans as f64).sqrt());
}
