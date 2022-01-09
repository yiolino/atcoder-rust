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
        s: Chars,
    }

    let mut vec = VecDeque::new();

    for si in &s {
        if *si == '0' {
            vec.push_back(*si);
        } else if *si == '1' {
            vec.push_back(*si);
        } else {
            vec.pop_back();
        }
    }

    let ans = vec.iter().collect::<String>();

    println!("{}", ans);
}
