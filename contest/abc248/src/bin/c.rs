use proconio::input;

const MOD: usize = 998244353;
fn main() {
    input!{
        n: usize,
        m: usize,
        k: usize,
    }

    // dp[i][j] = i番目（0 ~ n) までに総和がsumとなるものの数。
    let mut dp = vec![vec![0_usize; 3010]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..=k {
            for l in 1..=m {
                dp[i + 1][j + l] += dp[i][j];
                dp[i + 1][j + l] %= MOD;
            }
        }
    }

    let mut ans = 0;
    for i in 0..=k {
        ans += dp[n][i];
        ans %= MOD;
    }
    println!("{}", ans);
}
