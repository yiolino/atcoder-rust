use proconio::input;

fn main() {
    input!{
        n: usize,
    }

    let mut mat = vec![vec![0; 1509]; 1509];
    for _ in 0..n {
        input! {x: usize, y: usize};
        mat[x][y] += 1;
    }

    for i in 1..1501 {
        for j in 1..1501 {
            mat[i][j] += mat[i][j - 1];
        }
    }

    for i in 1..1501 {
        for j in 1..1501 {
            mat[j][i] += mat[j - 1][i];
        }
    }

    input! {q:usize};
    for _ in 0..q {
        input! {a: usize, b: usize, c: usize, d: usize};

        let ans = mat[c][d] + mat[a - 1][b - 1] - mat[a - 1][d] - mat[c][b - 1];
        println!("{}", ans);
    }
    

}
