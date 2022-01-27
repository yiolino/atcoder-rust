#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[allow(non_snake_case)]
fn main() {
    input!{
        _n: usize,
        s: String,
    }
    if s == "b".to_string() {
        println!("0");
        return;
    }

    let mut deque = VecDeque::new();
    deque.push_back('b');

    for i in 1..1000 {
        match i % 3 {
            1 => {deque.push_front('a'); deque.push_back('c')},
            2 => {deque.push_front('c'); deque.push_back('a')},
            _ => {deque.push_front('b'); deque.push_back('b')},
        }

        let tmp = deque.iter().collect::<String>();
        if tmp == s {
            println!("{}", i);
            return;
        }
    }

    println!("-1");
}
