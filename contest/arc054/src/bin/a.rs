use proconio::input;

fn main() {
    input! {
        l: f64,
        x: f64,
        y: f64,
        s: f64,
        d: f64,
    }

    let ans = if d >= s {
        let tmp = (d - s) / (x + y);
        let mut tmp2 = (l - d + s) / (y - x);
        if tmp2 < 0. {
            tmp2 = std::f64::MAX;
        }

        tmp.min(tmp2)
    } else {
        let mut tmp = (s - d) / (y - x);
        let tmp2 = (l - s + d) / (x + y);
        if tmp < 0. {
            tmp = std::f64::MAX;
        }

        tmp.min(tmp2)
    };

    println!("{}", ans);
}
