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
        m: usize,
    }

    let mut s = vec![];
    let mut c = vec![];

    for _ in 0..m {
        input! {mut ss: usize, cc: char,};

        ss -= 1;
        s.push(ss);
        c.push(cc);
    }

    // 0 ~ 1000までチェックする
    for ans in 0..1000 {
        let tmp = ans.to_string().chars().into_iter().collect::<Vec<char>>();
        if tmp.len() != n {
            continue;
        }
        let mut is_ok = true;

        for j in 0..m {
            let idx = s[j];
            if tmp[idx] != c[j] {
                is_ok = false;
            }
        }

        if is_ok {
            println!("{}", ans);
            return;
        }
    }

    println!("-1");
}
