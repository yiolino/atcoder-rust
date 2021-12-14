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
        c: usize,
        q: usize,
        mut tnc: [(usize, usize, usize,); q],
    }

    // 逆からクエリを流す
    tnc.reverse();

    // 縦に塗られた回数、横に塗られた回数をカウントしておく
    let mut count_one = 0;
    let mut count_two = 0;

    // その列、行が塗られたかどうかのboolean
    let mut painted_one = vec![false; 1_000_000_000];
    let mut painted_two = vec![false; 1_000_000_000];

    // 1~cのマスの個数
    let mut masu_count = vec![0; c];

    for i in 0..q {
        let t = tnc[i].0;
        let n = tnc[i].1 - 1;
        let c = tnc[i].2 - 1;

        if t == 1 {
            if painted_one[n] == false {
                masu_count[c] += w - count_two;

                painted_one[n] = true;
                count_one += 1;
            }

        } else {
            if painted_two[n] == false {
                masu_count[c] += h - count_one;

                painted_two[n] = true;
                count_two += 1;
            }
        }
    }

    let ans = masu_count
                        .iter()
                        .map(|u| u.to_string())
                        .join(" ");

    
    println!("{}", ans);
}
