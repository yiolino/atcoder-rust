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
        t: Chars,
    }

    let mut ans= "You can win";
    let atcoder: Vec<char> = "atcoder@".chars().collect();

    for (si, ti) in s.into_iter().zip(t) {
        if si == '@' {
            if atcoder.iter().all(|c| *c != ti) {
                ans = "You will lose";
                break;
            }
        } else if ti == '@' {
            if atcoder.iter().all(|c| *c != si) {
                ans = "You will lose";
                break;
            }
        } else {
            if si != ti {
                ans = "You will lose";
                break;
            }
        }
    }

    println!("{}", ans);
}
