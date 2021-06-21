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
        c: usize,
    }

    let max_yakusu = gcd(a, b);
    let max_yakusu = gcd(c, max_yakusu);

    let mut ans = 0;

    ans += a / max_yakusu - 1;
    ans += b / max_yakusu - 1;
    ans += c / max_yakusu - 1;

    println!("{}", ans);
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

