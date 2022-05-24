use proconio::input;

// 参考 https://qiita.com/u2dayo/items/f59a6661fc8037414a01#c問題super-ryuma
fn main() {
    input!{
        mut r1: i64,
        mut c1: i64,
        mut r2: i64,
        mut c2: i64
    }
    // まず原点に移動させる。
    r2 -= r1;
    c2 -= c1;
    r1 = 0;
    c1 = 0;

    // 1手、2手でいけるものを場合分け。それ以外は3手かかる。
    let ans = if r1 == r2 && c1 == c2 {
        0
    } else if r2+c2 == 0 || r2 - c2 == 0 || r2.abs() + c2.abs() <= 3 {
        1
    } else if (r2 + c2) % 2 == 0 || r2.abs() + c2.abs() <= 6 || (r2 + c2).abs() <=3 || (- r2 + c2).abs() <=3 {
        2
    } else {
        3
    };

    println!("{}", ans);
}
