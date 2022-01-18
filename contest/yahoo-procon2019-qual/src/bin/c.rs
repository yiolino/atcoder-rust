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
        k: usize,
        a: usize,
        b: usize,
    }

    let mut ans = 1;

    if a + 2 < b {
        // 何回叩けるかを考える。
        ans = (k + 1 - a) / 2 * (b - a) + a;
        if (k + 1 -a) % 2 != 0 {
            ans += 1;
        }
    } else {
        ans += k;
    }

    println!("{}", ans);
}
