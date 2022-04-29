use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: usize,        
    }

    let mut vec = vec![];

    for _ in 0..n {
        input! {mut s: Chars};
        s.reverse();
        vec.push(s);
    }

    vec.sort();
    for v in vec.iter_mut() {
        v.reverse();
        let ans = v.into_iter().join("");
        println!("{}", ans);
    }
}
