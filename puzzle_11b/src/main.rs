use std::cmp::min;
use std::cmp::max;

const EXPANSION_SIZE: u64 = 1000000-1;

fn main() {
    let grid = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // COSMIC EXPANSION!!!!
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();
    for i in 0..grid.len() {
        if grid[i].iter().all(|&c| c == '.') {
            empty_rows.push(i);
        }
        if grid.iter().all(|row| row[i] == '.') {
            empty_cols.push(i);
        }
    }

    // Collect all galaxies
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    // Find shortest paths between galaxies
    let mut path_sum: u64 = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for j in i+1..galaxies.len() {
            let galaxy_2 = &galaxies[j];
            // Find empty rows and cols between galaxies
            let empty_rows_between = empty_rows.iter().filter(|&&r| r > min(galaxy.0, galaxy_2.0) && r < max(galaxy.0, galaxy_2.0)).count();
            let empty_cols_between = empty_cols.iter().filter(|&&c| c > min(galaxy.1, galaxy_2.1) && c < max(galaxy.1, galaxy_2.1)).count();

            // Calculate path length (without adding the expansion)
            let y_steps: u64 = (galaxy_2.0 as i32 - galaxy.0 as i32).abs() as u64;
            let x_steps: u64 = (galaxy_2.1 as i32 - galaxy.1 as i32).abs() as u64;

            // Add the expansion: path length + the number of cells to add for every row/col
            path_sum += y_steps + x_steps + 
                (EXPANSION_SIZE * empty_rows_between as u64) as u64 + 
                (EXPANSION_SIZE * empty_cols_between as u64) as u64;
        }
    }
    println!("Path sum: {}", path_sum);
}
