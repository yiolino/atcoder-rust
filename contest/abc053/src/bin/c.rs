use proconio::input;

fn main() {
    input!{
        x: usize,
    }

    let mut ans = (x / 11) * 2;
    if x % 11 == 0 {
        ans += 0
    }
    else if x % 11 > 6 {
        ans += 2
    } else {
        ans += 1
    };

    println!("{}", ans);
}

