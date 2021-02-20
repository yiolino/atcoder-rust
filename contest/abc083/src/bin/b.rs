#[allow(unused_imports)]
use proconio::{input,fastout, marker::Chars};

#[fastout]
#[allow(non_snake_case)]
#[allow(unused_imports)]
fn main() {
    input!{
        N: i64,
        A: i64,
        B: i64,
    }

    let mut ans = 0;

    for mut i in 1..=N {
        let mut cand = 0;
        let ic = i;

        loop {
            cand += i % 10;
            i /= 10;
            if i == 0 {
                break;
            }
        }

        if A <= cand && cand <= B {
            ans += ic;
        }
    }

    println!("{}", ans);
}
