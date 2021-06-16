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
        a: usize,
        b: usize,
        c: usize,
    }

    let mod_4 = mod_pow(b, c, 4);
    let ans: usize;

    if mod_4 == 1 {
        ans = mod_pow(a, 1, 10);
    } else if mod_4 == 2 {
        ans = mod_pow(a, 2, 10);
    } else if mod_4 == 3 {
        ans = mod_pow(a, 3, 10);
    } else {
        ans = mod_pow(a, 4, 10);
    }

    println!("{}", ans);
}


// a: 底, n: 指数, m: mod
fn mod_pow<T>(mut a: T, mut n: T, m: T) -> T 
where
    T: num_traits::PrimInt,
{   
    let mut res = T::one();
    while n > T::zero() {
        if n & T::one() == T::one() {
            res = res * a % m;
        }
        a = a * a % m;
        n = n >> 1;
    }

    res
}