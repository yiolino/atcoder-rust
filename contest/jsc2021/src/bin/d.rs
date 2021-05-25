#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        P: usize,
    }

    let mut ans = P - 1;

    ans = ans * mod_pow(P - 2, N - 1, MOD);

    ans %= MOD;

    println!("{}", ans);
}


// mod_powの実装。型がprimitiveなら何でも取れるようにする。
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