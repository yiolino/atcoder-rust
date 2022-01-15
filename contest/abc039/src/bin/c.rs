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
    
    let mut do_seq: Vec<char> = "WBWBWWBWBWBW".chars().collect();
    do_seq.append(&mut do_seq.clone());
    do_seq.append(&mut do_seq.clone());

    let doremi = ["Do", "Do#", "Re", "Re#", "Mi", "Fa", "Fa#", "So", "So#", "La", "La#", "Si"];

    for i in 0..12 {
        let slice = do_seq[i..i+20].iter().collect::<String>();
        if slice == s {
            println!("{}", doremi[i]);
            return;
        }
        
    }
}
