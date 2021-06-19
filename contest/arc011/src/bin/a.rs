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
        m: usize,
        n: usize,
        l_n: usize,
    }

    let mut cnt = 0;
    let mut pool = 0;
    let mut sale = l_n;

    while sale + pool >= m {
        cnt += sale;
        let tmp = (sale + pool) / m * n;
        pool = (sale + pool) % m;

        sale = tmp;
    }

    cnt += sale;

    println!("{}", cnt);
}
