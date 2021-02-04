use proconio::{input,fastout};

#[fastout]
#[allow(unused_imports)]
#[allow(non_snake_case)]
fn main() {
    input!{
        A:i64,
        B:i64,
        _C:i64,
        K: usize,
    }

    let ans: i64;
    
    if K % 2 == 0 {
        ans = A - B;
    } else {
        ans = B - A;
    }

    println!("{}", ans);
}