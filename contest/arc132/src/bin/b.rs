use std::cmp::min;

use proconio::input;

fn main() {
    input!{
        n: usize,
        p: [usize; n],
    }

    let pos = p.iter().position(|&x| x == 1).unwrap();
    let next_pos = (pos + 1) % n;

    // 小さい順になっている場合
    let ans = if p[next_pos] == 2 {
        min(pos, n - pos + 2)
    } else {
        min(next_pos + 1, 1 + n - next_pos)
    };

    println!("{}", ans);
}
