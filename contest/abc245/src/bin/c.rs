use proconio::input;

fn main() {
    input!{
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n]
    }

    let mut dp  = vec![vec![false; 2]; n];

    dp[0][0] = true;
    dp[0][1] = true;

    for i in 1..n {
        if dp[i - 1][0] {
            if (a[i] - a[i - 1]).abs() <= k {
                dp[i][0] = true;
            }
            if (b[i] - a[i - 1]).abs() <= k {
                dp[i][1] = true;
            }
        }
        if dp[i - 1][1] {
            if (a[i] - b[i - 1]).abs() <= k {
                dp[i][0] = true;
            }
            if (b[i] - b[i - 1]).abs() <= k {
                dp[i][1] = true;
            }
        }
    }

    let ans = if dp[n-1][0] || dp[n-1][1] {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
