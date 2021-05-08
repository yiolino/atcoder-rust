#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        A: [i64; N],
    }

    let mut ans = std::i64::MAX;
    let sum:i64 = A.iter().sum();

    let mut head = 0;
    for i in 0..N {
        head += A[i];
        let tail = sum - head;
        ans = ans.min((tail - head).abs());
    }

    // for i in 0..N {
    //     let head:i64 = A.iter().enumerate().
    //                             filter(|(j, _a)| j <= &i).
    //                             map(|(_j, a)| *a)
    //                             .sum();
    //     let tail:i64 = A.iter().enumerate().
    //                             filter(|(j, _a)| j > &i).
    //                             map(|(_j, a)| *a)
    //                             .sum();
        
    //     ans = ans.min((head - tail).abs());               
    // }


    println!("{}", ans);
}
