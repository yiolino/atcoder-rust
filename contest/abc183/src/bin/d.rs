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
        w: i64,
        stp: [(usize, usize, i64); n],
    }

    // 速度で考える
    // 時刻s で +p
    // 時刻t で -p の変化が起きると考える
    let mut vec = vec![];
    for (s, t, p) in stp {
        vec.push((s, p));
        vec.push((t, -p));
    }

    vec.sort_by_key(|(time, p)| (*time, *p));

    let mut current = 0;
    for (_time, p) in vec {
        current += p;
        if current > w {
            println!("No");
            return;
        }
    }


    println!("Yes");
}
