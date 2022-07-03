use proconio::input;

fn main() {
    input!{
        n: usize,
        mut k: usize,
    }

    let mut k_vec = k.to_string().chars().collect::<Vec<char>>();
    k_vec.reverse();
    let mut r_k: usize = k_vec.iter().collect::<String>().parse().unwrap();

    if r_k < k {
        println!("0");
        return
    }

    let mut ans = 0;
    if k != r_k {
        while r_k <= n {
            ans += 1;
            r_k *= 10;
        }
    }

    while k <= n {
        ans += 1;
        k *= 10;
    }



    println!("{}", ans);
}
