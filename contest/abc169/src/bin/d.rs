use proconio::input;

fn main() {
    input!{
        n: i64,
    }

    let mut vec = prime_factorize(n);

    let mut ans = 0;
    for (_, ex) in vec.iter_mut() {
        for i in 1..1000i64 {
            if *ex >= i {
                ans += 1;
                *ex -= i;
            }
        }
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
    
    vec
}