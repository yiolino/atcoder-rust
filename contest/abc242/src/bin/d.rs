use proconio::{input, marker::{Usize1, Chars}};

fn main() {
    input!{
        s: Chars,
        q: usize,
    }

    for _ in 0..q {
        input! {t: usize, k: Usize1};
        // sの何番目から出発しているかという情報が必要
        let idx = k % 2_usize.checked_pow(t as u32).unwrap_or(k + 1);
        // idx をbitに見立てて、0と1の数をカウントする。
        let mut cnt_1 = 0;
        for i in 0..61 {
            if idx >> i & 1 > 0 {
                cnt_1 += 1;
            }
        }
        let cnt_0 = t - cnt_1;

        // Aを始点として、左に行くと（:=cnt_0) +1, 右に行くと (:=cnt_1) +2とする。
        let mut progress = cnt_0 + cnt_1 * 2;
        
        // s0の時の文字がBなら+1, Cなら+2
        let char_s0 = s[k / 2_usize.checked_pow(t as u32).unwrap_or(k + 1)];
        match char_s0 {
            'A' => (),
            'B' => progress += 1,
            'C' => progress += 2,
            _ => unreachable!(),
        }

        let ans = match progress % 3 {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            _ => unreachable!(),
        };

        println!("{}", ans)
    }
}
