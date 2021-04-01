#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
    }

    let mut cnt = 1;
    let mut divisors = vec![];
    while cnt as f64 <= (N as f64).sqrt() {
        if N % cnt == 0 {
            divisors.push((cnt, N / cnt));  // ペアをベクトルに保存
        }
        cnt += 1;
    }

    let mut ans = std::usize::MAX;

    for d in divisors {
        let del = d.0 - 1 + d.1 - 1;
        ans = ans.min(del);
    }

    println!("{}", ans);
}
