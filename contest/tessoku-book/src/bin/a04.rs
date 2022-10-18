use proconio::input;

fn main() {
    input!{
        mut n: usize,
    }

    let ans = format!("{:010b}", n);

    // let mut deq = VecDeque::new();
    // while n > 0 {
    //     if n % 2 == 1 {
    //         deq.push_front('1');
    //     } else {
    //         deq.push_front('0');
    //     }
    //     n /= 2;
    // }

    // for _ in 0..(10 - deq.len()) {
    //     deq.push_front('0');
    // }

    // let ans: String = deq.iter().join("");

    println!("{}", ans);
}
