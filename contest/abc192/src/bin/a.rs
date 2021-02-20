#[allow(unused_imports)]
use proconio::{input,fastout, marker::Chars};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: i64
    }

    let amari = N % 100;
    let ans: i64;
    if amari == 0 {
        ans = 100;
    } else {
        ans = 100 - amari;
    }

    println!("{}", ans);
}
