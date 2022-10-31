use proconio::{input, marker::Usize1};

fn main() {
    input!{
        h: usize,
        w: usize,
        mut x: [[usize; w]; h],
        q: usize,
    }

    // 横方向の累積
    for i in 0..h {
        for j in 1..w {
            x[i][j] += x[i][j - 1];
        }
    }

    // 縦方向の累積
    for i in 0..w {
        for j in 1..h {
            x[j][i] += x[j - 1][i];
        }
    }

    for i in 0..h {
        x[i].insert(0, 0);
    }

    x.insert(0, vec![0; w + 1]);

    for _ in 0..q {
        input! {a: usize, b: usize, c: usize, d: usize};

        let ans = x[c][d] + x[a - 1][b - 1] - x[c][b - 1] - x[a - 1][d];
        println!("{}", ans)
    }
}
