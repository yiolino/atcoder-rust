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
        x: [Chars; n],
    }

    let mut ans = 0;

    // xを数える
    for i in 0..n {
        for c in &x[i] {
            if *c == 'x' {
                ans += 1;
            }
        }
    }

    // 長押しを数える。
    for i in 0..9 {
        let mut push = false;
        for j in 0..n {
            if x[j][i] == 'o' {
                if !push {
                    push = true;
                }
            } else {
                if push {
                    ans += 1;
                }
                push = false;
            }
        }


        if push {
            ans += 1;
        }
    }

    println!("{}", ans);
}
