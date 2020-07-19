use std::cmp::{max, min};

#[derive(Debug)]
pub struct Board {
    pub points: Vec<(i64, i64)>,
}
#[allow(clippy::many_single_char_names)]
impl Board {
    pub fn draw(&self) {
        let mut b = 1i64 << 60;
        let mut t = -1i64 << 60;
        let mut l = 1i64 << 60;
        let mut r = -1i64 << 60;
        for (x, y) in &self.points {
            b = min(b, *x);
            t = max(t, *x);
            l = min(l, *y);
            r = max(r, *y);
        }
        let h = t - b + 1;
        let w = r - l + 1;
        let mut board = vec![vec![' '; w as usize]; h as usize];
        for (x, y) in &self.points {
            let x_ind = x - b;
            let y_ind = y - l;
            board[x_ind as usize][y_ind as usize] = '#';
        }
        println!();
        for raw in board {
            println!("{}", raw.into_iter().collect::<String>())
        }
        println!();
    }
}
