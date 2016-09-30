
pub mod solutions;
#[cfg(test)]
mod tests;

use std::io;

/// Code used for reading in generic input for Hacker Rank Problem Euler challenges.
pub fn read_hacker_rank () -> Vec<i64> {
    let mut t = String::new();
    io::stdin().read_line(&mut t)
        .expect("Failed to read input, T");
    let t: usize = match t.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number!")
    };
    let mut n = Vec::new();
    for _ in 0..t {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input, N");
        let buffer: i64 = match buffer
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_)  => panic!("Not a number.")
            };

        n.push(buffer);
    };
    n
}

// Code for reading in a square matrix of integers.
pub fn read_grid(col_len: usize) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for _ in 0..col_len {
        let mut row = String::new();
        io::stdin().read_line(&mut row)
            .expect("Failed to read input");
        let row: Vec<i32> = row
            .trim()
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        grid.push(row);
    }
    grid
}
