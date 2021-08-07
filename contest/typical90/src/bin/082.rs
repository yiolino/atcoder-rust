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
        l: usize,
        r: usize,
    }

    let p:usize = 1_000_000_000 + 7;


    let mut ans = 0_usize;
    for i in 1..=19u32 {
        let vr = min(10usize.pow(i) - 1, r);
        let vl = max(10usize.pow(i - 1), l);

        if vl > vr {
            continue;
        }

        let a:usize = summing_mod(vr, p);
        let b:usize = summing_mod((vl as i64 - 1) as usize, p);
        let ei = ((a as i64 - b as i64) + p as i64) as usize % p;
        ans += ei * i as usize;
        ans %= p;
    }

    println!("{}", ans);
}


fn summing_mod (n: usize, m: usize) -> usize {
    let v1 = n % m;
    let v2 = (n + 1) % m;
    let v = v1 * v2 % m;

    v * mod_inv(2, m) % m
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

// 逆元の計算
fn mod_inv(a: usize, m: usize) -> usize {
    let n = m - 2;
    
    mod_pow(a, n, m)
}