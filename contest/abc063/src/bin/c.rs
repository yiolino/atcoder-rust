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
        mut s: [usize; n],
    }

    let mut ans: usize = s.iter().sum();

    s.sort();

    for si in &s {
        if ans % 10 != 0 {
            println!("{}", ans);
            return;
        } 

        // 10の倍数は引いてはいけない
        if si % 10 != 0 {
            ans -= si;
        }
    }

    println!("{}", 0)
}
