use proconio::input;
use superslice::Ext;

fn main() {
    input!{
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut l1 = vec![0; n / 2];
    let mut l2 = vec![0; n - n / 2];
    for i in 0..n/2 {
        l1[i] = a[i];
    }
    for i in n/2..n {
        l2[i - n/2] = a[i];
    }

    let mut p = vec![];
    let mut q = vec![];

    for bit in 0..(1 << l1.len()) {
        let mut sum = 0;
        for i in 0..l1.len() {
            if bit >> i & 1 > 0 {
                sum += l1[i];
            }
        }

        p.push(sum)
    }
    for bit in 0..(1 << l2.len()) {
        let mut sum = 0;
        for i in 0..l2.len() {
            if bit >> i & 1 > 0 {
                sum += l2[i];
            }
        }

        q.push(sum)
    }

    q.sort();

    for i in 0..p.len() {
        let idx = q.lower_bound(&(k - p[i]));
        if idx < q.len() && q[idx] == k - p[i] {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
