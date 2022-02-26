use proconio::*;

const MOD: i64 = 1_000_000_007;

fn main() {
    input!{
        mut n: i64,
        mut m: i64,
    }

    if m > n {
        std::mem::swap(&mut n, &mut m);
    }

    if n - m > 1 {
        println!("0");
        return;
    }

    let mut ans = 1;

    for i in 1..=n {
        ans *= i;
        ans %= MOD;
    }

    for i in 1..=m {
        ans *= i;
        ans %= MOD;
    }

    if n - m == 0 {
        ans += ans;
        ans %= MOD;
    }

    println!("{}", ans);
}
