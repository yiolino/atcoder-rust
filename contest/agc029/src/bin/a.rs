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
        s: Chars
    }

    let mut num_b = 0;
    let mut cnt = 0;

    let n = s.len();
    for i in 0..n {
        if s[i] == 'B' {
            num_b += 1;
            cnt += i;
        }
    }

    let mut ans = 0;
    for i in (n - num_b)..n {
        ans += i;
    }

    println!("{}", ans - cnt);
}
