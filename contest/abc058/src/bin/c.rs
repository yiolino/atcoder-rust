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
        s: [Chars; n],
    }

    let mut vec = vec![0; 26];
    for c in &s[0] {
        let i = *c as u32 - 97;
        vec[i as usize] += 1;
    }

    for i in 1..n {
        let mut tmp_vec = vec![0; 26];
        for c in &s[i] {
            let i = *c as u32 - 97;
            tmp_vec[i as usize] += 1;
        }

        for (j, v) in tmp_vec.iter().enumerate() {
            vec[j] = min(*v, vec[j]);
        }
    }

    let ans = vec.iter()
        .enumerate()
        .filter(|(_i, n)| *n > &0)
        .map(|(i, n)| ((i as u8 + 97) as char).to_string().repeat(*n))
        .join("");

    println!("{}", ans)
}
