use proconio::*;
use std::collections::*;


fn main() {
    input! {
        n: usize,
        k: usize,
        p: [u32; n],
    }
    let mut h = BinaryHeap::new();
    for p in p.iter() {
        println!("{}", !*p);
        h.push(!*p);
        while h.len() > k {
            h.pop().unwrap();
        }
        if h.len() == k {
            println!("{}", !h.peek().unwrap());
        }
    }
}

