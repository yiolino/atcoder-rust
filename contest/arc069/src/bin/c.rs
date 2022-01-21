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
        s: usize,
        mut c: usize,
    }

    let mut ans = 0;

    if c / 2 > s {
        ans += s;
        c -= 2 * s;

        ans += c / 4;

    } else {
        ans += c / 2;
    }


    println!("{}", ans);
}
