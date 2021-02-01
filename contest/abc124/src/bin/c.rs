#[allow(unused_imports)]
use proconio::{input,fastout};

#[allow(non_snake_case)]
fn main() {
    input!{
        S: String,
    }

    let mut countzero = 0;
    let mut countone = 0;

    for (i, c) in S.chars().enumerate() {
        if i % 2 == 0 {
            if c != '0' {countone += 1}
        } else {
            if c != '1' {countone += 1}
        }
    }

    for (i, c) in S.chars().enumerate() {
        if i % 2 == 0 {
            if c != '1' {countzero += 1}
        } else {
            if c != '0' {countzero += 1}
        }
    }

    println!("{}", std::cmp::min(countzero, countone));
}
