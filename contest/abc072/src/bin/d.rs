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
        p: [usize; n],
    }

    let mut num_diff = 0;
    for (i, pi) in p.iter().enumerate() {
        if i+1 == *pi {
            num_diff += 1;
        }
    }

    let mut is_serial = false;
    let mut tmp_num_serial = 0;
    for (i, pi) in p.into_iter().enumerate() {
        if i+1 == pi {
            is_serial = true;
            tmp_num_serial += 1;
        } else {
            if is_serial {
                num_diff -= tmp_num_serial / 2;
                tmp_num_serial = 0;
            }
        }
    }

    num_diff -= tmp_num_serial / 2;
    

    println!("{}", num_diff);
}
