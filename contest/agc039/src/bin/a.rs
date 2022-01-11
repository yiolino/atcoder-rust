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
    input! {
        mut s: Chars,
        k: usize,
    }

    let is_all_same = (0..s.len()).all(|i| s[0] == s[i]);
    if is_all_same {
        println!("{}", k * s.len() / 2);
        return;
    }

    // 左右を繋げた時の連続した区間を計算
    let mut left = 0;
    let mut right = 0;
    for i in 0..s.len() {
        if s[i] == s[0] {
            left += 1;
        } else {
            break;
        }
    }
    for i in (0..s.len()).rev() {
        if s[i] == s[0] {
            right += 1;
        } else {
            break;
        }
    }

    // s内部の連続した区間の個数の総和を求める
    let mut inner_ran = 1;
    let mut innner_rans_vec = vec![];
    let mut prev = s[0];
    for i in 1..s.len() {
        if s[i] == prev {
            inner_ran += 1;
        } else {
            if inner_ran > 1 {
                innner_rans_vec.push(inner_ran);
                inner_ran = 1;
            }
            prev = s[i];
        }
    }
    if inner_ran > 1 {
        innner_rans_vec.push(inner_ran);
    }


    // まずs単体の答えを求める。
    let mut s_only: usize = 0;
    for inner in innner_rans_vec{
        s_only += inner / 2;
    }

    let ans = s_only * k - (left / 2 + right / 2 - (left + right) / 2) * (k - 1);
    
    println!("{}", ans)
}
