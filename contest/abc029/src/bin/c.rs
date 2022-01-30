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
    }

    f(n, vec![]);
}

// 再帰で解く
fn f(rest: usize, char_vec: Vec<char>) {
    if rest == 0 {
        println!("{}", char_vec.iter().collect::<String>());
        return;
    } else {
        for i in 0..3 {
            let c =  (97 + i) as u8 as char;
            let mut vec = char_vec.clone();
            vec.push(c);
            f(rest - 1, vec);
        }
    }

}