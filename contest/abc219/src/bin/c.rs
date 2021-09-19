#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::vec;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
    input!{
        x: Bytes,
        n: usize,
        mut s: [Bytes; n],
    }

    // アルファベットの順番が入れ替わったことを番号で示すvector
    // (例) aが5番目にある時
    // inv[b'a' - b'a'] = 4;
    let mut inv = vec![0; 26];
    for (i, xi) in x.iter().enumerate() {
        inv[(*xi - b'a') as usize] = i;
    }

    for si in s.iter_mut() {
        for b in si.iter_mut() {
            *b = inv[(*b - b'a') as usize] as u8;
        }
    }

    s.sort();

    for si in s {
        let ans = si
                        .into_iter()
                        .map(|i| x[i as usize] as char)
                        .collect::<String>();
        
        println!("{}", ans);
    }
}
