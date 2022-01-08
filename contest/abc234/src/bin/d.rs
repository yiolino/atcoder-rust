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
        k: usize,
        p: [usize; n],
    }

    let mut heap = BinaryHeap::new();

    for i in 0..k {
        heap.push(Reverse(p[i]));
    }

    println!("{}", heap.peek().unwrap().0);

    for i in k..n {
        if heap.peek().unwrap().0 < p[i] {
            heap.pop();
            heap.push(Reverse(p[i]));
            println!("{}", heap.peek().unwrap().0);
        } else {
            println!("{}", heap.peek().unwrap().0);
        }
    }
}
