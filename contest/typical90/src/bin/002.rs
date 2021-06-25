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
        let mut cnd: String = "".to_string();

        for j in (0..n).rev() {
            if i & 1 << j == 0 {
                cnd = cnd + "(";
            } else {
                cnd = cnd + ")";
            }
        }

        if judge(&cnd) {
            println!("{}", cnd);
        }
    }
}


fn judge(s: &String) -> bool {
    let mut dep: i64 = 0;
    let s_chars = s.chars().collect::<Vec<char>>();
    for i in 0..s.len() {
        if s_chars[i] == '(' {
            dep += 1;
        } else {
            dep -= 1;
        }

        if dep < 0 {
            return false;
        }
    }
    if dep == 0 {
        return true
    }

    false
}