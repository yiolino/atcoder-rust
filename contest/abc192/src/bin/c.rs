#[allow(unused_imports)]
use proconio::{input,fastout, marker::Chars};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: String,
        K: usize,
    }

    let mut a = N;

    for _i in 1..=K {
        a = f(a);
    }

    println!("{}", a);
}

#[allow(non_snake_case)]
fn f(N: String) -> String {
    let mut vec = Vec::new();
    for c in N.chars() {
        vec.push(c as i64 - 48);
    }
    vec.sort();

    let len = vec.len().to_string();
    let mut l: i64 = len.parse().unwrap();
    l -= 1;
    let mut g2 = 0;
    for n in &vec {
        g2 += 10i64.pow(l as u32) * n;
        l -= 1;
    }

    let len = vec.len().to_string();
    let mut l: i64 = len.parse().unwrap();
    l -= 1;
    let mut g1 = 0;
    for n in vec.iter().rev() {
        g1 += 10i64.pow(l as u32) * n;
        l -= 1;
    }

    (g1 - g2).to_string()
}