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
        x: usize,
        c_a: [[usize; m + 1]; n],
    }

    // bit全探索で解く
    let mut ans = std::usize::MAX;

    for bit in 0..1<<n {
        let mut rikai = vec![0; m];
        let mut tmp_ans = 0;
        for j in 0..n {
            if (1 << j) & bit > 0 {
                tmp_ans += c_a[j][0];
                for k in 0..m {
                    rikai[k] += c_a[j][k+1];
                }
            }
        }

        let is_over_x = rikai.iter().all(|r| *r >= x);
        if is_over_x {
            ans = ans.min(tmp_ans);
        }
    }

    
    if ans == std::usize::MAX {
        println!("-1");
        return;
    }
    println!("{}", ans);
}
