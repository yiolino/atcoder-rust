use proconio::input;

fn main() {
    input!{
        n:usize,
        array: [i64; n],
    }
    
    let sorted_idx = argsort(&array);
    let max = (array[sorted_idx[n - 1]], sorted_idx[n - 1]);
    let submax = (array[sorted_idx[n - 2]], sorted_idx[array.len() - 2]);

    for i in 0..n {
        if i != sorted_idx[n - 1] {println!("{}", max.0);}
        else {println!("{}",submax.0);}
    }
}


fn argsort<T: Ord>(v: &[T]) -> Vec<usize> {
    let mut idx = (0..v.len()).collect::<Vec<_>>();
    idx.sort_unstable_by(|&i, &j| v[i].cmp(&v[j]));
    idx
}