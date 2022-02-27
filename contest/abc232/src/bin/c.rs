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
use superslice::{Ext};

fn main() {
    input!{
        n: usize,
        m: usize,
    }

    let mut ab = vec![vec![false; n]; n];
    let mut cd = vec![vec![false; n]; n];

    for _ in 0..m {
        input! {mut a: usize, mut b: usize};
        a -= 1;
        b -= 1;
        ab[a][b] = true;
        ab[b][a] = true;
    }
    for _ in 0..m {
        input! {mut c: usize, mut d: usize};
        c -= 1;
        d -= 1;
        cd[c][d] = true;
        cd[d][c] = true;
    }

    let mut is_same = false;
    'outer: for p in (0..n).permutations(n) {
        for i in 0..n {
            for j in 0..n {
                if ab[i][j] != cd[p[i]][p[j]] {
                    continue 'outer;
                }
            }
        }

        is_same = true;
    }


    println!("{}", if is_same {"Yes"} else {"No"});
}
