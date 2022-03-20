use proconio::input;

fn main() {
    input!{
        n: usize,
        l: usize,
        r: usize
    }

    let mut ans = 0;
    for i in 0..60 {
        if n >> i & 1 == 1 {
            let upper = r.min((1 << (i + 1)) - 1);
            let lower = l.max(1 << i);

            if upper >= lower {
                ans += upper - lower + 1;
            }
        }
    }

    println!("{}", ans);
}
