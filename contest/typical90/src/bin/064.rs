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
        q: usize,
        a: [i64; n],
    }

    let mut ans = 0;
    let mut b = vec![];
    
    for i in 0..n-1 {
        b.push(a[i+1] - a[i]);
        ans += b[i].abs();
    }

    for _ in 0..q {
        input!{
            mut l: i64,
            mut r: i64,
            v: i64,
        }
        l -= 1;
        r -= 1;

        let mut mae = 0;
        let mut ato = 0;
        if l > 0 {
            mae += b[(l-1) as usize].abs();
            b[(l-1) as usize] += v;
            ato += b[(l-1) as usize].abs();
        }

        if r < n as i64 - 1 {
            mae += b[r as usize].abs();
            b[r as usize] -= v;
            ato += b[r as usize].abs();
        }

        ans += ato - mae;

        println!("{}", ans)
    }
}
