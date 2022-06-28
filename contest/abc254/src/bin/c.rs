use proconio::input;

fn main() {
    input!{
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    let mut tmp_a = a.clone();
    tmp_a.sort();
    for i in 0..k {
        let mut b = vec![];
        for r in (i..n).step_by(k) {
            b.push(a[r]);
        }
        
        b.sort();
        for r in (i..n).step_by(k) {
            a[r] = b[r / k];
        }

    }

    let ans = if tmp_a == a {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
