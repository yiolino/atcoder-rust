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
        s: Chars,
        t: Chars,
    }

    let mut match_num = 0;

    for i in 0..n {
        // if s[(n - 1 - i)..] == t[0..=i] {
        //     match_num = i + 1;
        // }
        let is_all_same = (0..=i).all(|j| s[n - 1 - i + j] == t[j]);
        
        if is_all_same {
            match_num = i + 1;   
        }
    }


    println!("{}", 2 * n - match_num);
}
