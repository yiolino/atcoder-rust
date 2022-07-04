use proconio::input;

fn main() {
    input!{
        n: usize,
        t: [usize; n],
    }

    // まずctz(a) = i となる min_aを求める
    let mut vec = vec![1_i64; 41];
    vec[1] = 0;
    for i in 2..=40 {
        let a = 1 << i;
        vec[i] = a;
    }

    let mut before = -1_i64;
    for ti in &t {
        // beforeより大きくなければならない。
        if *ti == 0 {
            if before % 2 == 0{
                vec[*ti] = before + 1;
            } else {
                vec[*ti] = before + 2;
            }
        } else {
            before = before >> ti;
            if before % 2 == 0{
                before += 1;
            } else {
                before += 2;
            }
            before = before << ti;
            vec[*ti] = before | 1_i64 << ti;
        }
    
        before = vec[*ti];
    }

    println!("{}", vec[t[n - 1]]);
}
