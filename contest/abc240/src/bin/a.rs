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
        a: i64,
        b: i64,
    }

    if b == 10 {
        if a == 1 || a == 9 {
            println!("Yes");
            return;
        } else {
            println!("No");
            return;
        }
    }

    if a == 1 {
        if b == 2 || a == 10 {
            println!("Yes");
            return;
        } else {
            println!("No");
            return;
        }
    }

    if b - a == 1 {
        println!("Yes");
        return;
    }

    println!("No");
}
