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
        a: usize,
        b: usize,
    }

    let r = b / gcd(a, b);

    if r > 1000_000_000_000_000_000 / a {
        println!("Large");
    } else {
        println!("{}", r * a);
    }
}


// ユークリッドの互助法による最大公約数
fn gcd(m:usize, n:usize) -> usize {
    // 再帰関数で実装する。
    // ベースケース
    if m % n == 0 {
        return n;
    }

    // 再帰呼び出し
    gcd(n, m % n)
}
