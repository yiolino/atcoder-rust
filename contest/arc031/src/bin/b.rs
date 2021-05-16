#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;


struct DfsBoard {
    w: usize,
    h: usize,
    board: Vec<Vec<char>>,
}

impl DfsBoard {
    fn dfs(&mut self, x: usize, y: usize) {
        self.board[x][y] = 'x';

        // 4方向の移動
        let dx = [-1, 0, 1, 0];
        let dy = [0, -1, 0, 1];

        for dir in 0..4 {
            let nx = (x as i64) + dx[dir];
            let ny = (y as i64) + dy[dir];

            // 座標の外に出たら終了
            if nx < 0 || ny < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if self.w <= nx || self.h <= ny {
                continue;
            }

            // 移動先が x ならスルー
            if self.board[nx][ny] == 'x' {
                continue;
            }

            // 再帰
            self.dfs(nx, ny);

        }
    }
    
    fn is_ok(&self) -> bool {
        for hi in 0..self.h {
            for wi in 0..self.w {
                // 一続きではなかった場合陸が残っている
                if self.board[hi][wi] == 'o' {
                    return false;
                }
            }
        }

        true
    }
}



#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        field: [Chars; 10],
    }

    let h = 10;
    let w = 10;

    // たかだか100マスなので、もし陸になったら？を全通り試す
    // もともと1つの島の場合も引っかかるので、海の場合だけ考えれば良い
    for hi in 0..h {
        for wi in 0..w {
            if field[wi][hi] == 'x' {
                let mut board = field.clone();
                board[wi][hi] = 'o';
                let mut board = DfsBoard {w, h, board};
                board.dfs(wi, hi);

                if board.is_ok() {
                    println!("YES");
                    return;
                }
            }
        }
    }

    println!("NO");
}
