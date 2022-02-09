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
        a: usize, 
        b: usize,
    }

    if a > b {
        println!("0");
        return;
    }

    if n == 1 {
        if a != b {
            println!("0");
            return;
        } 
        if a == b {
            println!("1");
            return;
        }
    }


    println!("{}", b * (n - 2) - a * (n - 2) + 1);
}
