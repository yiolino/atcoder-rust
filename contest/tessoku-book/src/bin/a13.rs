use proconio::input;

fn main() {
    input!{
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut r = vec![0; n];

    for i in 0..n {
        if i == 0 {
            r[i] = 0;
        } else {
            r[i] = r[i - 1];
        }

        while r[i] < n - 1 && a[r[i] + 1] - a[i] <= k {
            r[i] += 1;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        ans += r[i] - i;
    }

    println!("{}", ans);
}
