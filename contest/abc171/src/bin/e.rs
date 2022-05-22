use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let s = a.iter().fold(0, |acc, x| acc ^ x);
    let vec = a.iter().map(|x| x ^ s).collect::<Vec<usize>>();
    let ans: String = vec
        .into_iter()
        .map(|x| x.to_string())
        .join(" ");

    println!("{}", ans);
}
