use num_integer::gcd;
use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    for d in (1..=r - l).rev() {
        for x in l..=r - d {
            let y = x + d;
            if gcd(x, y) == 1 {
                println!("{}", d);
                return;
            }
        }
    }
}
