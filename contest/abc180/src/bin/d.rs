use proconio::input;

fn main() {
    input!{
        mut x: u128,
        y: u128,
        a: u128,
        b: u128
    }

    let mut ans = 0;

    // aがbを追い抜かすまではaでカウントする。
    while x * a < x + b && x * a < y {
        x *= a;
        ans += 1;
    }

    println!("{}", ans + ((y - 1 - x) / b));
}
