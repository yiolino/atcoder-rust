use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        q: usize
    }

    let mut front = vec![-1; n+1];
    let mut end = vec![-1; n+1];

    for _ in 0..q {
        input! {c: usize};
        match c {
            1 => {
                input! {x: i64, y: i64};
                front[y as usize] = x;
                end[x as usize] = y;
            },
            2 => {
                input! {x: i64, y: i64};
                front[y as usize] = -1;
                end[x as usize] = -1;
            },
            3 => {
                input! {x: i64};
                let mut now = x;
                let mut top = -1;
                while now != -1 {
                    top = now;
                    now = front[now as usize];
                }

                let mut vec = vec![];
                let mut now = top;
                while now != -1 {
                    vec.push(now);
                    now = end[now as usize];
                }
                print!("{} ", vec.len());
                let ans = vec.iter().join(" ");
                println!("{}", ans);
            },
            _ => unreachable!(),
        }
    }
}
