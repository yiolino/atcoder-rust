#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        S: Chars,
    }

    let mut ans = "AC";
    let mut count_flg = false;
    let l = S.len();

    for (i, c) in S.iter().enumerate() {
        if i == 0 {
            if *c != 'A' {
            ans = "WA";
            }
        } else if i >= 2 && i <= l-2 {
            if *c == 'C' {
                if count_flg {ans = "WA";} else {count_flg = true;}
            } else {
                if c.is_uppercase() {ans = "WA";}
            }
        } else {
            if c.is_uppercase() {ans = "WA";}
        }
    }
    
    if !count_flg {ans = "WA"}
    
    println!("{}", ans);
}
