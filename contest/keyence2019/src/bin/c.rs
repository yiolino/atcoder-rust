use std::cmp::Reverse;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }

    // a と b の差分を考えていく
    let mut ans = 0;
    let mut debt = 0;
    let mut plus_vec = vec![];

    for (a, b) in a.into_iter().zip(b) {
        if a - b < 0 {
            ans += 1;
            debt += b - a;
        } else if b == a {
            continue;
        } else {
            plus_vec.push(a - b);
        }
    }

    plus_vec.sort_by_key(|x| Reverse(*x));

    plus_vec.insert(0, 0);
    for i in 1..plus_vec.len() {
        plus_vec[i] += plus_vec[i-1];
    }

    for i in 0..plus_vec.len() {
        if plus_vec[i] >= debt {
            ans += i;
            println!("{}", ans);
            return;
        }
    }

    println!("-1");
}
