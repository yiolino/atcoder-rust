use num_integer::gcd;
use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = a[0];

    for a in a {
        ans = gcd(ans, a);
    }

    println!("{}", ans);
}