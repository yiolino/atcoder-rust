#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::vec;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input!{
        s: Chars,
    }

    let n = s.len() + 1;
    let mut a = vec![0; n];

    for i in 0..n-1 {
        if s[i] == '<' {
            a[i+1] = a[i] + 1;
        }
    }


    for i in (0..n-1).rev() {
        if s[i] == '>' {
            a[i] = max(a[i],a[i+1] + 1);
        }
    }

    let ans: usize = a.into_iter().sum();

    println!("{}", ans);
}
