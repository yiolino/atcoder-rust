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
        h: usize,
        w: usize,
        mut a: [Chars; h],
    }

    // まず行から削除
    for i in (0..h).rev() {
        if a[i].iter().all(|c| c == &'.') {
            a.remove(i);
        }
    }

    // 列を削除
    for i in (0..w).rev() {
        let mut is_all_dot = true;

        for j in 0..a.len() {
            if a[j][i] == '#' {
                is_all_dot = false;
            }
        }

        if is_all_dot {
            for j in 0..a.len() {
                a[j].remove(i);
            }
        }
    }

    for i in 0..a.len() {
        let ans = a[i].iter().join("");
        println!("{}", ans);
    }
}
