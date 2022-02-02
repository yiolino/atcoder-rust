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
        l: usize,
        r: usize,
    }

    let mut ans = std::usize::MAX;

    for i in l..l+2019 {
        for j in i+1..l+2019 {
            if i > r || j > r {
                continue;
            }

            ans = ans.min(i * j % 2019);
        }
    }

    println!("{}", ans);
}
