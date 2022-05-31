const MOD: i64 = 1_000_000_007;

fn main() {
    proconio::input!(n: usize, mut a: [i64; n]);

    // dp[i][j] := i個の+/-を-が連続しないよう並べる方法の数
    // j = 0: 最後が -
    // j = 1:        +
    let mut dp = vec![vec![0; 2]; n+1];
    dp[0][1] = 1;
    for i in 1..=n {
        dp[i][0] = dp[i-1][1] % MOD;
        dp[i][1] = (dp[i-1][0] + dp[i-1][1]) % MOD;
    }

    let mut ans = 0;
    for i in 0..n {
        let mut t = 0;
        if i == 0 {
            t += dp[n-1][0] + dp[n-1][1]; // n-1個の符号の並べ方
        } else {
            t += dp[i][1] * dp[n-i][1];
            t -= dp[i][0] * dp[n-i][0];
            t %= MOD;
        }
        ans += a[i] * t;
        ans %= MOD;
        if ans < 0 {
            ans += MOD;
        }
    }
    println!("{}", ans);
}

