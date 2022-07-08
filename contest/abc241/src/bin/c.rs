use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: usize,
        s: [Chars; n],
    }

    for x in 0..n {
        for y in 0..n {
            for (dx, dy) in &[(1, 0), (0, 1), (1, 1), (1, -1)] {
                if calc(x as i32, y as i32, *dx, *dy, &s) {
                    println!("Yes");
                    return ;
                }
            }
        }
    }

    println!("No");
}

fn calc(mut x: i32, mut y: i32, dx: i32, dy: i32, s: &Vec<Vec<char>>) -> bool {
    let mut cnt = 0;
    let n = s.len() as i32;

    for _ in 0..6 {
        if !(x.min(y) >= 0 && x.max(y) < n) {
            return false;
        }
        if s[x as usize][y as usize] == '#' {
            cnt +=1;
        }

        x += dx;
        y += dy;
    }

    cnt >= 4
}