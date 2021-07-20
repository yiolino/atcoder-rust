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
        mut k: usize,
    }

    let mut ans = 1;
    let modd = 1_000_000_000 + 7;

    if k == 1 {
        if n != 1 {
            println!("0");

        } else {
            println!("{}", k);
        }
        return;
        
    } else if k == 2 {
        if n > 2 {
            println!("0");

        } else {
            for i in 0..n {
                ans *= k - i;
            }
            println!("{}", ans);

        }
        return;
    
    } else {
        if n <= 2 {
            for i in 0..n {
                ans *= k - i;
            }
        } else {
            ans *= k;
            ans *= k - 1;
            ans %= modd;

            ans *= mod_pow(k-2, n-2, modd);
            ans %= modd;
        }

        println!("{}", ans);
    }
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
