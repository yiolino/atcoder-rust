#[allow(non_snake_case)]
#[allow(unused_imports)]

use proconio::{input,fastout};


#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: i64,
    }

    let mut memo = vec![-1; (N + 1) as usize];

    println!("{}", lucas(N, &mut memo));
}

#[allow(non_snake_case)]
fn lucas(N: i64, memo:&mut Vec<i64>) -> i64 {
    if N == 0 {2}
    else if  N == 1 {1}
    else {
        if memo[N as usize] as i64 != -1  {memo[N as usize]}
        else {
            memo[N as usize] = lucas(N - 1, memo) + lucas(N - 2, memo);
            memo[N as usize]
        }
    }
}