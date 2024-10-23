#![allow(dead_code)]
use crate::helpers::gen_vec;
use crate::helpers::shuffle;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Table {
    data: [[u8; 9]; 9],
}

impl Table {
    pub fn new() -> Table {
        let mut data = [[0; 9]; 9];
        data[0] = gen_vec(1, 9).try_into().unwrap();
        shuffle(&mut data[0]);
        Table { data }
    }
    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.data[y][x] = value;
    }
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.data[y][x]
    }
    pub fn solve(&mut self) -> bool {
        for y in 0..9 {
            for x in 0..9 {
                if self.data[y][x] == 0 {
                    for n in 1..=9 {
                        if self.is_valid(x, y, n) {
                            self.set(x, y, n);
                            if self.solve() {
                                return true;
                            }
                            self.set(x, y, 0);
                        }
                    }
                    return false;
                }
            }
        }
        true
    }
    pub fn stringify(&self) -> String {
        self.data
            .iter()
            .map(|row| {
                row.iter()
                    .map(u8::to_string)
                    .collect::<Vec<String>>()
                    .join(" ")
                    + "\n"
            })
            .collect::<String>()
    }
    pub fn is_valid(&self, x: usize, y: usize, n: u8) -> bool {
        if self.data[y].contains(&n) || (0..9).any(|i| self.data[i][x] == n) {
            return false;
        }
        let (x0, y0) = (x / 3 * 3, y / 3 * 3);
        (0..3).all(|i| (0..3).all(|j| self.data[y0 + i][x0 + j] != n))
    }
    pub fn remove_cells(&mut self, n: usize) {
        let mut rng = rand::thread_rng();
        loop {
            let (x, y) = (rng.gen_range(0..9), rng.gen_range(0..9));
            self.data[y][x] = 0;
            if self.num_of_empty() >= n {
                break;
            }
        }
    }
    pub fn num_of_empty(&self) -> usize {
        self.data.iter().flatten().filter(|&&x| x == 0).count()
    }
}
