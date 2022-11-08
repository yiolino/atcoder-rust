use proconio::input;

fn main() {
    input!{
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }

    let mut mat = vec![vec![0; 1509]; 1509];

    for (a, b, c, d) in abcd {
        mat[a][b] += 1;
        mat[c + 1][d + 1] += 1;
        
        mat[a][d + 1] -= 1;
        mat[c + 1][b] -= 1;
    }

    // 累積和をとる
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

    for i in 1..=h {
        let mut v = vec![];
        for j in 1..=w {    
            v.push(mat[i][j].to_string());
        }
        
        println!("{}", v.join(" "));
    }
}
