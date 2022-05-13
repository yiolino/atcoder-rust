use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: usize,
        s: Chars
    }

    let mut num_rga = vec![0_usize, 0_usize, 0_usize];
    for si in s.iter() {
        match *si {
            'R' => num_rga[0] += 1,
            'G' => num_rga[1] += 1,
            'B' => num_rga[2] += 1,
            _ => unreachable!(),
        }
    }
    let all = num_rga.iter().fold(1, |acc, x| acc * x);

    // all から j-i = k-j である個数を引く。補集合として考える。
    // i と j を固定すると k = 2j - i
    let mut co_set = 0;
    for i in 1..n {
        for j in i+1..n {
            let k = 2 * j - i;
            if k > n {
                continue;
            }

            if s[i-1] != s[j-1] && s[j-1] != s[k-1] && s[i-1] != s[k-1] {
                co_set += 1;
            }
        }
    }

    println!("{}", all - co_set);
}
