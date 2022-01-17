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
        mut h: [usize; n],
    }
    
    h.push(0);

    let mut ans = 0;
    while h.iter().sum::<usize>() > 0 {
        for i in 0..n {
            if h[i] > 0 && h[i + 1] == 0 {
                ans += 1;
            }
            if h[i] > 0 {
                h[i] -= 1;
            }
        }
    }

    println!("{}", ans);
}
