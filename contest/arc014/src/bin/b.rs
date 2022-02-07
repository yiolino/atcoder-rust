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
        w: [Chars],
    }

    let mut set = HashSet::new();

    for (i, wi) in w.iter().enumerate() {
        if set.contains(wi) {
            if i % 2 == 0 {
                println!("LOSE");
                return;
            } else {
                println!("WIN");
                return;
            }
        } else {
            set.insert(wi);
        }

        if i != 0 {
            if &w[i][0] != w[i-1].last().unwrap() {
                if i % 2 == 0 {
                    println!("LOSE");
                    return;
                } else {
                    println!("WIN");
                    return;
                }
            }
        }
    }

    println!("DRAW")
}
