use proconio::input;

fn main() {
    input!{
        n: usize,
        mut b: [usize; n],
    }

    let mut ans = vec![];
    while b.len() > 0 {
        let mut vec = vec![];
        for (i, bi) in b.iter().enumerate().rev() {
            if (i + 1) == *bi {
                vec.push(i);
                ans.push(i+1);
                break;
            }
        }

        if vec.len() == 0 {
            break;
        } else {
            for v in vec {
                b.remove(v);
            }
        }
    }

    if b.len() == 0 {
        for a in ans.iter().rev() {
            println!("{}", a);
        }
    } else {
        println!("-1");
    }
}
