use proconio::input;

fn main() {
    input!{
        n: usize,
        k: usize,
        a: [usize; n]
    }

    // 累積和を求める
    let mut cum = vec![0; n + 1];
    for i in 1..n+1 {
        cum[i] = cum[i - 1] + a[i - 1]
    }

    let mut r = vec![0; n];

    for i in 0..n {
        if i == 0 {
            r[i] = 0;
        } else {
            r[i] = r[i - 1];
        }

        while r[i] < n && cum[r[i] + 1] - cum[i] <= k {
            r[i] += 1;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        ans += r[i] - i;
    }

    println!("{}", ans);
}
