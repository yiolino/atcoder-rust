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
        mut n: usize,
    }

    let mut prime_vec = vec![];
    let mut cnd = 2;

    while cnd * cnd <= n {
        if n % cnd == 0 {
            prime_vec.push(cnd);
            n /= cnd;

        } else {
            cnd += 1;
        }
    }

    if n != 1 {
        prime_vec.push(n);
    }

    let mut ans = 0;
    let mut cnt = 1;

    while cnt < prime_vec.len() {
        cnt *= 2;
        ans += 1;
    }

    println!("{}", ans);

}
