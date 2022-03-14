use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: usize,
        k: usize,
        rsp: [usize; 3],
        t: Chars,
    }

    // r := 0, s := 1, p := 2 とする
    let mut t_num = vec![];
    for t in t {
        match t {
            'r' => t_num.push(0),
            's' => t_num.push(1),
            'p' => t_num.push(2),
            _ => unreachable!(),
        }
    }


    let mut ans = 0_usize;
    let mut vec:Vec<usize> = vec![];
    
    for (i, t_n) in t_num.iter().enumerate() {
        if i < k {
            vec.push(*t_n);
            ans += rsp[(t_n + 2) % 3];
            continue;
        } else if i >= n - k {
            if vec[i - k] == *t_n {
                continue;
            }
            ans += rsp[(t_n + 2) % 3];
        } else {
            let mut te = *t_n;
            if vec[i - k] == te {
                te += 1;
                if t_num[i + k] == te {
                    te += 1;
                }
                vec.push(te);
            } else {
                ans += rsp[(t_n + 2) % 3];
                vec.push(te);
            }
        }
    }

    println!("{}", ans);
}
