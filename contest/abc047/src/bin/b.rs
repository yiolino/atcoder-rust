use proconio::input;

fn main() {
    input!{
        w: i64,
        h: i64,
        n: usize,
    }

    // x と y それぞれでupper と lower を管理
    let mut x_lu = (0, w);
    let mut y_lu = (0, h);

    for _ in 0..n {
        input! {x: i64, y: i64, a: usize};
        match a {
            1 => x_lu.0 = x_lu.0.max(x),
            2 => x_lu.1 = x_lu.1.min(x),
            3 => y_lu.0 = y_lu.0.max(y),
            4 => y_lu.1 = y_lu.1.min(y),
            _ => unreachable!(),
        }
    }

    let ans = if x_lu.1 - x_lu.0 > 0 && y_lu.1 - y_lu.0 > 0 {
        (x_lu.1 - x_lu.0) * (y_lu.1 - y_lu.0)
    } else {
        0
    };
    
    println!("{}", ans);
}
