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
        mut n: usize,
    }

    let mut vec = vec![];
    let letters = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();


    while n > 0 {
        n -= 1;
        let amari = n % 26;
        vec.push(letters[amari]);

        n /= 26;
    }

    vec.reverse();
    let ans = vec.iter().join("");

    println!("{}", ans);
}
