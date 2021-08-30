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
        n: usize,
        m: usize,
    }

    // m個のDequeを格納したvector。各Dequeが筒を表す。
    let mut tube: Vec<VecDeque<usize>> = vec![VecDeque::new(); m];
    for i in 0..m {
        input!(k: usize);
        for _ in 0..k {
            input!(mut a: usize);
            a -= 1;
            tube[i].push_back(a);
        }
    }

    let mut available = vec![-1; n];
    let mut deque: VecDeque<(usize, usize)> = VecDeque::new();

    for ai in 0..m {
        set_available(&mut tube, &mut available, &mut deque, ai); 
    }

    while !deque.is_empty() {
        let (ai, aj) = deque.pop_front().unwrap();
        set_available(&mut tube, &mut available, &mut deque, ai);
        set_available(&mut tube, &mut available, &mut deque, aj);
    }

    for i in 0..m {
        if !tube[i].is_empty() {
            println!("No");
            return;
        }
    }

    println!("Yes");
}



// 筒：A -> queを格納したvector
// available -> vector. i番目の色が先頭にいるかどうか。初期値は-1
fn set_available(tube: &mut Vec<VecDeque<usize>>, available: &mut Vec<i32>, deque: &mut VecDeque<(usize, usize)>, ai: usize) {
    // tube[ai]が空なら何もしない。

    match tube[ai].pop_front() {
        Some(i) => if available[i] >= 0 {
                            deque.push_back((available[i] as usize, ai));
                        } else {
                            available[i] = ai as i32;
                        },
        None => (),
    }
}
