use proconio::{input, marker::Bytes};

fn main() {
    input!{
        n: usize,
        a: [Bytes; n],
    }
    

    let n = n as i64;
    let direction = vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, -1), (-1, 1)];

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            for d in &direction {
                let mut x = i;
                let mut y = j;
                let mut number = a[x as usize][y as usize] as i64 - 48;
                for _ in 0..n-1 {
                    x = (x + d.0 + n) % n;
                    y = (y + d.1 + n) % n;
                    number *= 10;
                    number += a[x as usize][y as usize] as i64 - 48;
                }

                ans = ans.max(number);
            }
        }
    }

    println!("{}", ans);
}
