use num_integer::lcm;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: usize,
        b: usize,
    }

    let sum_all = n * (n + 1) / 2;

    let num_a = n / a;
    let sum_a = a * (num_a * (num_a + 1) / 2);
    let num_b = n / b;
    let sum_b = b * (num_b * (num_b + 1) / 2);

    let c = lcm(a, b);
    let num_c = n / c;
    let sum_c = c * (num_c * (num_c + 1) / 2);
    
    let ans = sum_all + sum_c - sum_a - sum_b;

    println!("{}", &ans);
}
