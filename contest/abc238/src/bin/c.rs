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
        n: String,
    }

    let keta = n.clone().chars().collect::<Vec<char>>().len();
    let mut ans = 0;

    for k in 1..keta {
        ans += sum_f(k);
    }

    let n = n.parse::<usize>().unwrap();
    let upper = f(n) % 998244353;

    ans += (upper * (upper + 1) / 2) % 998244353;
    ans %= 998244353;

    
    println!("{}", ans);
}

fn f(n: usize) -> usize {
    if n < 10 {
        n
    } else {
        let k = n.clone().to_string().chars().collect::<Vec<char>>().len();
        let a = 10i64.pow((k-1) as u32) as usize;

        n - a + 1
    }
} 

fn sum_f(k: usize) -> usize {
    let upper = 10i64.pow(k as u32) as usize - 1;
    let upper_f = f(upper) % 998244353;

    (upper_f * (upper_f + 1) / 2) % 998244353
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_f() {
        assert_eq!(f(9), 9);
        assert_eq!(f(11), 2);
        assert_eq!(f(16), 7);
    }

    #[test]
    fn test_fn_sum_f() {
        assert_eq!(sum_f(2), 4095);
    }
}