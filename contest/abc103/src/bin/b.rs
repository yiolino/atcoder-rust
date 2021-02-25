#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(non_snake_case)]
fn main() {
    input!{
        S: String,
        T: String,
    }

    let mut S: Vec<char> = S.chars().collect();
    let T : Vec<char> = T.chars().collect();

    let l: usize = S.len();

    for _ in 0..l {
        let next_S = rotate(&mut S);
        let matching = next_S.iter().zip(&T).filter(|&(a, b)| a == b).count();
        if matching == l {
            println!("Yes");
            std::process::exit(0);
        }
    }

    println!("No");
}


fn rotate(vec: &mut Vec<char>) -> &mut Vec<char> {
    let last = vec.pop().unwrap();
    vec.insert(0, last);

    vec
}