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

    let mut vec:Vec<usize> = vec![];

    for _ in 0..n {
        input!{
            a: [usize; 6],
        }

        vec.push(a.iter().sum());
    }

    let mut ans = 1;
    for v in vec {
        ans *= v;
        ans %= 1_000_000_000 + 7;
    }

    println!("{}", ans);
}
