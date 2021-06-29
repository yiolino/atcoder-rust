#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        n: usize,
    }

    for i in 0..1<<n {
        let mut cnd = "".to_string();

        for j in 0..n {
            if i & 1<<j == 0 {
                cnd = "(".to_string() + &cnd;
            } else {
                cnd = ")".to_string() + &cnd;
            }
        }

        if judge(&cnd) {
            println!("{}", cnd);
        }
    }
}


// 入力が正しいカッコ列であるかの判定
// ( と ) の累積を考える
fn judge(s: &String) -> bool {
    let mut deps: i64 = 0; // 累積のカウンター

    let s_chars = s.chars().collect::<Vec<char>>();

    if s.len() == 0 {
        return false;
    }

    for i in 0..s_chars.len() {
        if s_chars[i] == '(' {
            deps += 1;
        } else {
            deps -= 1;
        }

        if deps < 0 {
            return false;
        }
    }

    if deps == 0 {
        return true;
    }

    false
}