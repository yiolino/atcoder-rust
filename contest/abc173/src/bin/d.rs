use std::{cmp::Reverse};
use proconio::input;


fn main() {
    input!{
        n:usize,
        mut a: [usize; n],
    }

    a.sort_by_key(|x| Reverse(*x));

    let mut ans = 0;

    let mut t = n - 1;
    for i in 0..n {
        let mut limit = 2;
        if i == 0 {
            limit = 1;
        }
        for _ in 0..limit {
            if t > 0 {
                ans += a[i];
                t -= 1;
            }
        }
    }
    println!("{}", ans);
}
