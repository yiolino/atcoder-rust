use proconio::{input, marker::Chars};

fn main() {
    input!{
        s: Chars,
    }

    let mut ans = 0;
    for w in s.windows(2) {
        if w[0] != w[1] {
            ans += 1;
        }    
    }

    println!("{}", ans);
}
