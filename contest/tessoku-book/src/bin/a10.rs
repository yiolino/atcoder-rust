use std::cmp::max;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        a: [usize; n],
        d: usize,
    }

    let mut p = vec![0; n];
    p[0] = a[0];
    for i in 1..n {
        p[i] = a[i].max(p[i - 1]);
    }

    let mut q = vec![0; n];
    q[n-1] = a[n-1];
    for i in (0..n-1).rev() {
        q[i] = a[i].max(q[i + 1]);
    }

    for _ in 0..d {
        input! {l: Usize1, r: Usize1};

        let ans = max(p[l - 1], q[r + 1]);
        println!("{}", ans);
    }
}
