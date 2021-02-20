#[allow(unused_imports)]
use proconio::{input,fastout, marker::Chars};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        S: Chars,
    }

    let mut ans = "Yes";
    let mut i = 1;
    for c in S {
        if i % 2 != 0 {
            if !(c as i32 >= 97 && c as i32 <= 122) {
                ans = "No";
                break;
            } 
        }
        if i % 2 == 0 {
            if !(c as i32 >= 65 && c as i32 <= 90) {
                ans = "No";
                break;
            }
        }

        i += 1;
    }

    println!("{}", ans);
}
