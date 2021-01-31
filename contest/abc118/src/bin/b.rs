use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
    }

    let mut count_vec = vec![0; m];

    for _ in 0..n {
        input! {
            k: usize,
            a: [usize; k],
        }

        for ai in a {
            count_vec[ai - 1] += 1;
        }
    }

    let mut ans = 0;
    for c in count_vec {
        if c == n {ans += 1;}
    }

    println!("{}", ans);
}
