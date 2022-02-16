use proconio::*;

fn main() {
    input! {
        mut n: usize,
        a: [usize; 5],
    }

    let min = a.iter().min().unwrap();

    let ans = (n + min - 1)/ min + 4;

    println!("{}", ans);
}
