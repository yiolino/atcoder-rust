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
        mut s: Chars,
        n: usize,
    }

    for _ in 0..n {
        input! {mut l: usize, mut r: usize};
        l -= 1;
        r -= 1;

        let mut tmp = vec![];
        for i in l..=r {
            tmp.push(s[i]);
        }

        tmp.reverse();

        for (zero_i, i) in (l..=r).enumerate() {
            s[i] = tmp[zero_i];
        }
    }

    let ans = s.into_iter().collect::<String>();

    println!("{}", ans);
}
