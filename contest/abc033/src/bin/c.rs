use proconio::{input, marker::Chars};

fn main() {
    input!{
        s: Chars,
    }

    let mut ans = 0;
    let mut is_contain_zero = false;
    for si in s {
        if si == '0' {
            is_contain_zero = true;
        }
        if si == '+' {
            if !is_contain_zero {
                ans += 1;
            }
            is_contain_zero = false;
        }
    }

    if !is_contain_zero {
        ans += 1;
    }

    println!("{}", ans);
}
