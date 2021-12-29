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

    // 100文字なので全てのパターンを試す
    let n = s.len();

    for i in 0..n {
        for j in i..n {

            let pre = &s[..i];
            let suf = &s[j..n];

            let mut string = String::new();

            string.push_str(pre);
            string.push_str(suf);

            if string == "keyence" {
                println!("YES");
                return;
            }
        }
    }

    println!("NO");
}
