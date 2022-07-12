use std::mem::swap;

use proconio::input;

fn main() {
    input!{
        x: i64,
        a: i64,
        d: i64,
        n: i64,
    }

    let mut min = a;
    let mut max = a + d * (n - 1);
    if min > max {
        swap(&mut min, &mut max);
    }

    let ans = if x < min {
        min - x
    } else if x > max {
        x - max
    } else {
        let mut tmp = x - min;
        if d != 0 {
            tmp %= d.abs();
        }

        tmp.min(d.abs() - tmp)
    };

    println!("{}", ans);
}
