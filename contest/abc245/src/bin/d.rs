use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        a: [i64; n + 1],
        c: [i64; n + m + 1],
    }

    let mut b = vec![0; m + 1];
    b[m] = c[n + m] / a[n];

    for i in 1..=m {
        let mut sum = 0;
        for j in 0..i {
            let ai = if n + j >= i {
                a[n + j - i]
            } else {
                0
            };
            sum += b[m - j] * ai;
        }
        b[m - i] = (c[n + m - i] - sum) / a[n];
    }

    println!("{}", b.iter().join(" "));
}
