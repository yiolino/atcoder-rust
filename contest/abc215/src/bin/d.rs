use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
    }

    let mut is_gcd_eq1 = vec![true; 100005];

    for _ in 0..n {
        input! {a: usize};

        let p = prime_factorize(a);
        for pi in p {
            if is_gcd_eq1[pi] {
                for i in (pi..100005).step_by(pi) {
                    is_gcd_eq1[i] = false;
                }
            }
        } 
    }


    let mut res = vec![];
    for i in 1..=m {
        if is_gcd_eq1[i] {
            res.push(i);
        }
    }

    println!("{}", res.len());
    for r in res {
        println!("{}", r);
    }
}

// 1以上n以下の整数が素数かどうか返す
fn prime_factorize(mut n: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            n /= i;
            res.push(i);
        }
        i += 1;
    }

    if n != 1 {
        res.push(n);
    }

    res
}
