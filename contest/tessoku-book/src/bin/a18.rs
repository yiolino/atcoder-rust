use proconio::input;

fn main() {
    // dp二次元配列を考える
    // dp[i][j] = カード1 ~ iの中から何枚かを選び、選んだカードに書かれた整数の合計をjにすることは可能か
    input! {
        n:  usize,
        s:  usize,
        a:  [usize; n],
    }

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..(n + 1) {
        for j in 0..(s + 1) {
            if dp[i - 1][j] {
                dp[i][j] = dp[i - 1][j]
            }
            let ai = a[i - 1];
            if j >= ai && dp[i - 1][j - ai] {
                dp[i][j] = dp[i - 1][j - ai]
            }
        }
    }

    let ans = if dp[n][s] { "Yes" } else { "No" };

    println!("{}", ans);
}
