#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: i64,
        mut A: [i64; N],
    }

    let mut sum_Ai2 = 0;
    let mut Ai2 = 0;
    
    for i in 0..N {
        sum_Ai2 += A[i as usize] * A[i as usize];
        Ai2 += A[i as usize];
    }

    let ans = N * sum_Ai2 - Ai2 * Ai2;

    println!("{}", ans);
}
