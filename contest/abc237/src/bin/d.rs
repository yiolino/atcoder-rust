use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: usize,
        s: Chars,
    }

    let mut vec = vec![(-2, -2); n + 1];
    vec[0] = (-1, n as i64 + 1);

    for (i, si) in s.iter().enumerate() {
        let tmp = vec[i];
        if *si == 'L' {
            vec[i].0 = i as i64 + 1;
            vec[i + 1] = (tmp.0, i as i64);
            if tmp.0 != -1 {
                vec[tmp.0 as usize].1 = i as i64 + 1;
            }
        } else {
            vec[i].1 = i as i64 + 1;
            vec[i + 1] = (i as i64, tmp.1);
            if tmp.1 != n as i64 + 1 {
                vec[tmp.1 as usize].0 = i as i64 + 1;
            }
        }
    }

    let mut next = 0;
    for (i, v) in vec.iter().enumerate() {
        if v.0 == -1 {
            next = i;
        }
    }

    let mut ans = vec![];
    loop {
        ans.push(next.to_string());
        next = vec[next].1 as usize;
        if next == n + 1 {
            break;
        }
    }

    println!("{}", ans.join(" "));
}
