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
        n: usize,
    }

    let mut x = 0;
    let mut vec = vec![];

    for _ in 0..n {
        input!{
            a: i64,
            b: i64,
        }
        x -= a;
        vec.push(2 * a + b);
    }

    vec.sort();
    
    let mut ans = 0;
    while x <= 0 {
        let tmp = vec.pop().unwrap();
        x += tmp;
        ans += 1;
    }

    println!("{}", ans);
}
