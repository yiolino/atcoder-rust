use proconio::{input, marker::Chars};

fn main() {
    input!{
        mut n: Chars
    }

    let mut ans = 0;
    for (i, n) in (n.iter()).rev().enumerate() {
        if *n == '1' {
            ans += 1 << i;
        }
    }

    println!("{}", ans);
}
