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
        d: [usize; n],
        m: usize,
        t: [usize; m],
    }

    let mut map = HashMap::new();
    for di in d {
        *map.entry(di).or_insert(0) += 1;
    }

    let mut ans = "YES";
    for ti in t {
        match map.get_mut(&ti) {
            Some(v) => if *v > 0 {
                *v -= 1
            } else {
                ans = "NO"; break;
            },
            None => {ans = "NO"; break;},
        }
    }

    println!("{}", ans);
}
