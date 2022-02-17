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
        mut a: [usize; n],
        mut b: [usize; m],
    }

    if n < m {
        println!("NO");
        return;
    }

    a.sort();
    a.reverse();
    b.sort();
    b.reverse();

    for i in 0..m {
        if b[i] > a[i] {
            println!("NO");
            return;
        }
    }

    println!("YES");
}
