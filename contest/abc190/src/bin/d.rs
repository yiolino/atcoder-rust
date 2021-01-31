use proconio::input;

fn main() {
    input! {
        mut n: usize
    }

    while n % 2 == 0 {n /= 2;}
    let mut ans = 0;

    let mut cntr = 1;
    while cntr * cntr <= n {
        if n % cntr == 0 {ans += 2;}
        if cntr * cntr == n {ans -= 1;}
        cntr += 1;
    }

    println!("{}", ans * 2);
}
