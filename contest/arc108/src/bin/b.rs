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
        _n: usize,
        s: Chars,
    }

    let mut tmp = vec![];

    for si in s {
       tmp.push(si);

        if tmp.len() >= 3 {
            if tmp[(tmp.len() - 3)..].to_vec() == vec!['f', 'o', 'x'] {
                for _ in 0..3 {
                    tmp.pop();
                }
            }
        }
    }

    println!("{}", tmp.len());
}
