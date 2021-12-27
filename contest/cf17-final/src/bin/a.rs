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
        mut s: Chars,
    }

    let mut ans = "NO";

    if s.len() > 9 {
        println!("{}", ans);
        return;
    }

    for _ in 0..9 {
        s.push(' ');
    }

    let idx = vec![0, 4, 6, 8];

    for i in idx {
        // if i > s.len() {
        //     continue;   
        // }
        if s[i] != 'A' {
            s.insert(i, 'A');
        }
    }
    
    if s.into_iter().filter(|&c| c != ' ').collect::<String>() == "AKIHABARA" {
        ans = "YES";
    }

    println!("{}", ans);
}
