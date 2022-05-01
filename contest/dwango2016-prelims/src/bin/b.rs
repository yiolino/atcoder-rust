use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        k: [usize; n - 1],
    }

    let mut ans = vec![];
    for i in 0..n-2 {
        ans.push(k[i].min(k[i + 1]));
    }

    ans.insert(0, k[0]);
    ans.push(*k.last().unwrap());

    println!("{}", ans.into_iter().join(" "));
}
