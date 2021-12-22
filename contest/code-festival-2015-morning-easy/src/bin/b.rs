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
        s: Chars,   
    }

    let mut ans = -1;

    if n % 2 == 0 {
        ans = 0;
        let center = n / 2;

        for i in 0..center {
            if s[i] != s[i+center] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
