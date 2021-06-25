#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        mut n: String,
        k: usize,
    }

    for _ in 0..k {
        n = usize_to_base9(base8_to_usize(n));
        
        let mut n_chars = n.chars().collect::<Vec<char>>();
        for i in 0..n_chars.len() {
            if n_chars[i] == '8' {
                n_chars[i] = '5';
            }
        }

        n = n_chars.iter().collect();
    }

    println!("{}", n);
}


// 8進数から10進数へ
fn base8_to_usize(n: String) -> usize {
    let mut ans = 0;
    let mut x = 1;
    let m = n.len();
    let n_chars = n.chars().collect::<Vec<char>>();

    for i in (0..m).rev() {
        ans += 1_usize * (n_chars[i] as usize - '0' as usize) * x;
        x *= 8_usize;
    }

    ans
}

// 10進数から9進数へ
fn usize_to_base9(mut n: usize) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let mut ans: String = "".to_string();
    while n > 0 {
        ans = (n % 9).to_string() + &ans;
        n /= 9;
    }

    ans
}