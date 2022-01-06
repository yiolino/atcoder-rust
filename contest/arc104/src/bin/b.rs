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

    let mut ans = 0;

    for i in 0..n {
        let mut c1 = 0;
        let mut c2 = 0;

        for j in i..n {
            match s[j] {
                'A' => c1 += 1,
                'T' => c1 -= 1,
                'G' => c2 += 1,
                'C' => c2 -= 1,
                _ => (),
            }

            if c1 == 0 && c2 == 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
