use num_integer::lcm;
use proconio::input;

fn main() {
    input!{
        n: usize,
        t: [usize; n],
    }

    let mut ans = 1_usize;

    for t in t {
        ans = lcm(t, ans);
    }

    println!("{}", ans);
}
