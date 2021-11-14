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
        s: [usize; n],
    }

    let mut vec = [false; 1001];

    for a in 1..=150 {
        for b in 1..=150 {
            let menseki = 4 * a * b + 3 * a + 3 * b;
            if menseki > 1000 {
                continue;
            } else {
                vec[menseki as usize] = true;
            }
        }
    }

    let mut ans = 0;
    for si in s {
        if !vec[si] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
