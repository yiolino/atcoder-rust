use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let ymd = s
        .split('/')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();
    let (mut y, mut m, mut d) = (ymd[0], ymd[1], ymd[2]);
    let days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    while y % (m * d) != 0 {
        let is_leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
        d += 1;
        let max = days[m - 1] + if m == 2 && is_leap { 1 } else { 0 };
        if d > max {
            d = 1;
            m += 1;
            if m > 12 {
                m = 1;
                y += 1;
            }
        }
    }
    println!("{:04}/{:02}/{:02}", y, m, d);
}