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
        m: usize,
    }

    for _ in 0..m {
        input!(mut s: Chars);

        s.reverse();

        for si in s {
            let a = match si {
                'A' => 'T',
                'T' => 'A',
                'G' => 'C',
                'C' => 'G',
                _ => '_',
            };
            
            print!("{}", a);
        }
        println!("");
    }
}
