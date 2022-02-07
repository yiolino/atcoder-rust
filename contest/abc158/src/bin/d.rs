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
        mut s: Chars,
        q: usize,
    }

    let mut deque : VecDeque<char> = s.into_iter().collect();
    let mut is_flip = false;

    for _ in 0..q {
        input! {t: usize};
        if t == 1 {
            is_flip = !is_flip;
            
        } else {
            input! {f: usize, c: char};
            if f  == 1 {
                if is_flip {
                    deque.push_back(c);
                } else {
                    deque.push_front(c);
                }
            } else {
                if is_flip {
                    deque.push_front(c);
                } else {
                    deque.push_back(c);
                }
            }
        }
    }

    let ans: String;
    if is_flip {
        ans = deque.iter().rev().join("");
    } else {
        ans = deque.iter().join("");
    }

    println!("{}", ans);
}




// deque を使っていないヴァージョン
// fn main() {
//     input!{
//         mut s: Chars,
//         q: usize,
//     }

//     let mut ex_front = vec![];
//     let mut ex_back = vec![];
//     let mut is_flip = false;

//     for _ in 0..q {
//         input! {t: usize};
//         if t == 1 {
//             is_flip = !is_flip;
//         } else {
//             input! {f: usize, c: char};
//             if f == 1 {
//                 if is_flip {
//                     ex_back.push(c);
//                 } else {
//                     ex_front.insert(0,c);
//                 }
//             } else {
//                 if is_flip {
//                     ex_front.insert(0, c);
//                 } else {
//                     ex_back.push(c);
//                 }
//             }
//         }
//     }

//     if is_flip {
//         ex_back.reverse();
//         s.reverse();
//         ex_front.reverse();

//         println!("{}", ex_back.iter().join("").to_string() + &s.iter().join("") + &ex_front.iter().join(""));

//     } else {
//         println!("{}", ex_front.iter().join("").to_string() + &s.iter().join("") + &ex_back.iter().join(""));
//     }