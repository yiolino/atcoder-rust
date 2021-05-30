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
        N: usize,
    }

    let mut vec = vec![];

    let mut now = 0;
    for i in 1..=N {
        if is_pow2(i) {
            now += 1;
        }
        vec.push(now);
    }

    let ans = vec.into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");

    println!("{}", ans);
}


// 2の累乗かどうかの判定
fn is_pow2<T>(x: T) -> bool
where
    T: num_traits::PrimInt,
{
    if x == T::zero() {
        return false
    } else {
        return (x & x - T::one()) == T::zero()
    }
}