use std::cmp::Reverse;

use proconio::input;

fn main() {
    input!{
        n: usize,
        t: i64, 
    }

    let mut vec = vec![];
    let mut total = 0;

    for _ in 0..n {
        input! {a: i64, b: i64};
        total += a;
        vec.push(a - b);
    }

    vec.sort_by_key(|w| Reverse(*w));

    let mut ans = 0;

    for v in vec {
        if total <= t {
            break;
        }

        total -= v;
        ans += 1;
    }

    if total > t {
        ans = -1;
    }

    println!("{}", ans);
}
