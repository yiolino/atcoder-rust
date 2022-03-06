use proconio::input;

fn main() {
    input!{
        n: usize,
        l: [i64; n],
    }

    // 累積和で考える
    let mut memo = vec![0; 2010];
    let mut cum = vec![0; 2010];

    for i in &l {
        memo[*i as usize] += 1;
    }

    // cum[i] := [0, i) の積和
    for i in 1..cum.len() {
        cum[i] = cum[i - 1] + memo[i - 1];
    }

    // a < b < c と考える -> これはだめで、a == b == c の場合を忘れている。
    let mut ans = 0_usize;
    for a in 0..n {
        for b in 0..n {
            if a == b {
                continue;
            }
            let min = (l[a] - l[b]).max(l[b] - l[a]);
            let max = l[a] + l[b];

            let mut cnt = cum[max as usize] - cum[(min + 1) as usize];

            if min < l[a] && l[a] < max {
                cnt -= 1;
            }
            if min < l[b] && l[b] < max {
                cnt -= 1;
            }

            ans += cnt;
        }
    }

    ans /= 6;
    println!("{}", ans);
}
