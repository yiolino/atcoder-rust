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

    let n = s.len() - 1;
    let mut ans = 0;
    for (i, si) in s.into_iter().enumerate() {
        match si {
            'U' => {
                ans += n - i;
                ans += 2 * i;
            },
            'D' => {
                ans += 2 * (n - i);
                ans += i;
            },
            _ => unreachable!(),
        }
    }

    println!("{}", ans);
}
