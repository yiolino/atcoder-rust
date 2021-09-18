#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1};
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
        s1: String,
        s2: String,
        s3: String,
        t: Chars,
    }

    for c in t {
        if c == '1' {
            print!{"{}", s1};
        } else if c == '2' {
            print!{"{}", s2};
        } else {
            print!{"{}", s3};
        }
    }

    println!("");
}
