const MOD: i64 = 1_000_000_007;

fn main() {
    proconio::input!(n: usize, mut a: [i64; n]);

    // dp[i][j] := i個の+/-を-が連続しないよう並べる方法の数
    // j = 0: 最後が -
    // j = 1:        +
    let mut dp = vec![vec![0; 2]; n];
    dp[0][1] = 1;
    for i in 1..n {
        dp[i][0] = dp[i - 1][1] % MOD;
        dp[i][1] = dp[i - 1][0] + dp[i - 1][1] % MOD;
    }

    // ai 以前の +/- の並べ方の場合の数と、以降の並べ方の数を掛け算したものを考える。
    let mut ans = 0;
    for i in 0..n {
        let mut t = 0;
        if i == 0 {
            t += dp[n - 1][0] + dp[n - 1][1];
        } else {
            t += dp[i][1] * dp[n - i][1]; // + で終わった箇所 * 残り n-i個が + で終わる場合の数（反転させて繋げるイメージ）
            t -= dp[i][0] * dp[n - i][0];
            t %= MOD;
        }
        ans += t * a[i];
        ans %= MOD;
    }

    println!("{}", ans)
}

