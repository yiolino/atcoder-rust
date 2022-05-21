use std::cmp::max;

use proconio::input;

fn main() {
    input!{
        x1: isize,
        y1: isize,
        r: isize,
        x2: isize,
        y2: isize,
        x3: isize,
        y3: isize,
    }

    // 円が内包されているか
    let is_red = x2 <= x1 - r && x1 + r <= x3 && y2 <= y1 - r && y1 + r <= y3;
    // 正方形が内包されているか
    let is_blue =  r * r < max((x2 - x1).pow(2), (x3 - x1).pow(2)) + max((y2 - y1).pow(2), (y3 - y1).pow(2));

    println!("{}", if !is_red { "YES" } else { "NO" });
    println!("{}", if is_blue { "YES" } else { "NO" });
}
