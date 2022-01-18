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
        mut a: [usize; n],
    }

    let mut odd = vec![];
    let mut even = vec![];

    for i in 0..n {
        if i % 2 == 0 {
            even.push(a[i]);
        } else {
            odd.push(a[i]);
        }
    }

    let ans = if n % 2 != 0 {
        even.reverse();
        even.append(&mut odd);
        even.iter().join(" ")
    } else {
        odd.reverse();
        odd.append(&mut even);
        odd.iter().join(" ")
    };


    println!("{}", ans);
}
