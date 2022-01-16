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
        r: i64,
        g: i64,
        b: i64,
        n: i64,
    }

    let r_max = n / r;
    let g_max = n / g;

    let mut ans = 0;
    for i in 0..=r_max {
        for j in 0..=g_max {
            let nokori = n - i * r - j * g;
            if nokori >= 0 && nokori % b == 0 {
                ans += 1;
            } 
        }
    }


    println!("{}", ans);
}
