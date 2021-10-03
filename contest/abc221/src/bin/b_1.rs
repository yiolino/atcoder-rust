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

#[fastout]
fn main() {
    input!{
        s: Chars,
        t: Chars,
    }

    let mut s_vec = vec![];
    let mut t_vec = vec![];

    let n = s.len();
    
    for i in 0..n {
        if s[i] != t[i] {
            s_vec.push((i, s[i]));
            t_vec.push((i, t[i]));
        }
    }


    if s_vec.len() == 0 {
        println!("Yes");
    } else if s_vec.len() == 2 {
        if s_vec[0].0+1 == s_vec[1].0 && s_vec[0].1 == t_vec[1].1 && s_vec[1].1 == t_vec[0].1 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
