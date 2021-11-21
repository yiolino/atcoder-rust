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
        p: [[i64; 3]; n],
    }

    let mut idx_3sum = vec![];

    for i in 0..n {
        let sum: i64 = p[i].iter().sum();
        idx_3sum.push((i+1, sum));
    }

    let original_idx_3sum = idx_3sum.clone();
    idx_3sum.sort_by_key(|s| s.1);
    idx_3sum.reverse();
    idx_3sum.insert(0, (0,0));

    let kth_score = idx_3sum[k].1;

    for (_, score) in original_idx_3sum.iter() {
        if kth_score - score <= 300 {
            println!("Yes");
        } else {
            println!("No");
        }
    }

}
