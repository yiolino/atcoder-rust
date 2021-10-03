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
        mut s: Chars,
        mut t: Chars,
    }

    let mut ans = "No";

    if s == t {
        ans = "Yes";
    }

    for i in 0..s.len()-1 {
        let tmp_i = s[i];
        let tmp_ii = s[i+1];
        s[i+1] = tmp_i;
        s[i] = tmp_ii;
        if s == t {
            ans = "Yes";
        }
        s[i] = tmp_i;
        s[i+1] = tmp_ii;
    } 


    println!("{}", ans);
}
