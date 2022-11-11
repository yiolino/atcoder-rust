use std::vec;

use proconio::input;

fn main() {
    input!{
        n: usize,
    }

    let mut mat = vec![vec![0; 1509]; 1509];

    for _ in 0..n {
        input! {mut a: usize, mut b: usize, mut c: usize, mut d: usize};
        a += 1;
        b += 1;
        c += 1;
        d += 1;

        mat[a][b] += 1;
        mat[a][d] -= 1;
        mat[c][b] -= 1;
        mat[c][d] += 1;
    }

    for i in 1..1509 {
        for j in 1..1509 {
            mat[i][j] += mat[i][j - 1];
        }
    }

    for i in 1..1509 {
        for j in 1..1509 {
            mat[j][i] += mat[j - 1][i];
        }
    }

    let mut ans = 0;
    for i in 0..1509 {
        for j in 0..1509 {
            if mat[i][j] > 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
