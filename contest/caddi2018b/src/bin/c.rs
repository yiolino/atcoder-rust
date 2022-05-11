use proconio::input;

fn main() {
    input!{
        n: usize,
        p: i64,   
    }

    let prime = prime_factorize(p);

    let mut ans = 1;
    for pr in prime {
        let e = pr.1 as usize / n;
        ans *= pr.0.pow(e as u32);
    }
    
    println!("{}", ans);
}


fn prime_factorize(mut n: i64) -> Vec<(i64, i64)> {
    let mut vec = vec![];

    let mut cnt = 2;
    while cnt * cnt <= n {
        if n % cnt != 0 {
            cnt += 1;
            continue;
        }
        let mut ex = 0; // 指数
        while n % cnt == 0 {
            ex += 1;
            n /= cnt;
        }
        vec.push((cnt, ex));

        cnt += 1;
    }   

    // 最後に残った数について
    if n != 1 {
        vec.push((n, 1));
    }
    
    vec.sort_by_key(|x| x.1);
    vec
}