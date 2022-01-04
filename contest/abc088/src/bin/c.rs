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
        c: [[i64; 3]; 3],
    }

    let mut a = vec![0; 3];
    let mut b = vec![0; 3];

    for i in 0..3 {
        b[i] = c[0][i] - a[0];
    }
    for i in 0..3 {
        a[i] = c[i][0] - b[0];
    }        

    let mut ans = "Yes";

    for i in 0..3 {
        for j in 0..3 {
            if c[i][j] != a[i] + b[j] {
                ans = "No";
            }
        }
    }

    println!("{}", ans);
}
