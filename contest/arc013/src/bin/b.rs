use std::cmp::Reverse;
use proconio::input;

fn main() {
    input!{
        c: usize,
    }

    let mut max_sizes = vec![0; 3];

    for _ in 0..c {
        input! {mut nml: [usize; 3]};
        nml.sort_by_key(|w| Reverse(*w));

        for (i, ni) in nml.iter().enumerate() {
            max_sizes[i] = max_sizes[i].max(*ni);
        }
    }

    let ans = max_sizes.iter().fold(1, |acc, x| acc * x);
    println!("{}", ans);
}
