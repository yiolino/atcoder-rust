use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [usize; n],
        mut b: [usize; n],
    }

    // ai ⊕ bi = x <=> ai ⊕ (ai ⊕ bi) = ai ⊕ x <=> bi = ai ⊕ x
    // よって、ci = ai ⊕ x となるような ci を全て列挙して、数列cとし、cとbを比較すれば良い。
    // xの候補はa1とb1 ~ bnを比較していけば良い。
    b.sort();
    let mut ans = vec![];
    for bi in b.iter() {
        let x = a[0] ^ bi;
        let mut c = vec![];
        for ai in a.iter() {
            c.push(ai ^ x);
        }
        c.sort();
        if c == b {
            ans.push(x);
        }
    }

    ans.sort();
    ans.dedup();

    println!("{}", &ans.len());
    for ans_i in ans {
        println!("{}", ans_i)
    }
}
