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
        t: Chars,
    }

    if s.len() != t.len() {
        println!("-1");
        return;
    }

    let n = s.len();
    
    // i: 始点
    for i in 0..n {
        let mut cnt = 0;
        let mut is_same = true;

        while cnt < n {
            
            let idx = (i + cnt) % n;
            if s[cnt] != t[idx] {
                is_same = false;
                break;
            }
            
            cnt += 1;
        }

        if is_same {
            println!("{}", i);
            return;
        }
    }

    println!("-1");
}
