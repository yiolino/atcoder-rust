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
    }

    let mut chr_vec = vec![];
    let mut num_vec = vec![];

    chr_vec.push(s[0]);
    let mut len = 1;

    for i in 1..s.len() {
        if s[i] == chr_vec.last().cloned().unwrap() {
            len += 1;
        } else {
            chr_vec.push(s[i]);
            num_vec.push(len);
            len = 1;
        }
    }
    num_vec.push(len);

    let ans = chr_vec.into_iter().zip(num_vec.into_iter())
                .map(|(c, n)| c.to_string() + &n.to_string())
                .join("");

    println!("{}", ans);
}
