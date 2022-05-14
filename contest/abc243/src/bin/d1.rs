use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input!{
        _n: usize,
        x: usize,
        s: Chars
    }

    let mut deque = VecDeque::new();
    for si in s {
        if let Some(l) = deque.back() {
            if (*l == 'R' || *l == 'L') && si == 'U' {
                deque.pop_back();
            } else {
                deque.push_back(si);
            }
        } else {
            deque.push_back(si);
        }
    }

    let mut ans = x;
    for d in deque {
        match d {
            'U' => ans /= 2,
            'L' => ans *= 2,
            'R' => {ans *= 2; ans += 1},
            _ => unreachable!(),
        }
    }

    println!("{}", ans);
}
