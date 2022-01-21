#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input!{
        mut n: usize,   
    }

    let ans = if is_prime(n) {
        "Prime"
    } else if is_like_prime(n) {
        "Prime"
    } else {
        "Not Prime"
    };

    println!("{}", ans);
}


fn is_prime(n: usize) -> bool {
    if n == 1 {
        return false;
    }

    for j in 2..=n {
        if j * j > n {
            break;
        }

        if n % j == 0 {
            return false;
        }
    }

    true
}

fn is_like_prime(mut n: usize) -> bool {
    if n == 1 {
        return false;
    }
    
    let kurai_1 = n % 10;

    let is_not_even5 = kurai_1 % 2 != 0 && kurai_1 != 5;

    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }

    let cannot_div_3 = sum % 3 != 0;

    is_not_even5 && cannot_div_3
}