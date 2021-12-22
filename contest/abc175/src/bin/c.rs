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
        mut x: i64,
        k: i64,
        d: i64,
    }

    // 対称性より正にしても問題ない
    if x < 0 {
        x *= -1;
    }

    // 原点との差をdで割った時の商と余りを考える
    let sho = x / d;
    let amari = x % d;

    let ans;
    if sho >= k {
        ans = x - k * d;

    } else {
        if (k - sho) % 2 == 0 {
            ans = amari;
        } else {
            ans = d - amari;
        }
    }

    println!("{}", ans);
}
