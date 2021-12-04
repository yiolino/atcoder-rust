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
        s: String,
    }

    let one: String = "oxx".to_string().repeat(4);
    let two: String = "xxo".to_string().repeat(4);
    let three: String = "xox".to_string().repeat(4);

    let m_one: Vec<&str> = one.matches(&s).collect();
    let m_two: Vec<&str> = two.matches(&s).collect();
    let m_three: Vec<&str> = three.matches(&s).collect();

    let mut ans = "No";

    if m_one.len() > 0 {
        ans = "Yes";
    }
    if m_two.len() > 0 {
        ans = "Yes";
    }
    if m_three.len() > 0 {
        ans = "Yes";
    }


    println!("{}", ans);
}
