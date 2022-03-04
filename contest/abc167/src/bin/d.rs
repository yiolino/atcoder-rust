use proconio::*;

const LOG:usize = 60;

fn main() {
    input!{
        n: usize,
        k: usize,
        a: [marker::Usize1; n],
    }

    // ダブリングで解いていく
    // db_table[i][j] := 町jの2^i個上の親 
    let mut db_table = vec![vec![0; n]; LOG + 1];

    // 2^0 == 1 個上の親
    db_table[0] = a;

    for i in 0..LOG {
        for j in 0..n {
            db_table[i+1][j] = db_table[i][db_table[i][j]];
        }
    }

    // 町0から出発してk回テレポートした先
    // kを二進数展開して計算していく
    let mut ans = 0;
    for i in  0..=LOG {
        if 1 & k>>i > 0 {
            ans = db_table[i][ans];
        }
    }

    println!("{}", ans + 1)  // 1-indexに戻す
}
