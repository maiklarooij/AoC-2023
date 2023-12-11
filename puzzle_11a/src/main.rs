fn main() {
    let grid = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // COSMIC EXPANSION!!!!
    let mut updated_grid = grid.clone();
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
    empty_rows.iter().for_each(|&i| updated_grid.insert(i + empty_rows.iter().position(|&r| r == i).unwrap(), vec!['.'; updated_grid[0].len()]));
    empty_cols.iter().for_each(|&i| updated_grid.iter_mut().for_each(|row| row.insert(i + empty_cols.iter().position(|&r| r == i).unwrap(), '.')));

    // Collect all galaxies
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for i in 0..updated_grid.len() {
        for j in 0..updated_grid[0].len() {
            if updated_grid[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    // Find shortest paths between galaxies
    let mut path_sum: u32 = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for j in i+1..galaxies.len() {
            let galaxy_2 = &galaxies[j];
            let y_steps: u32 = (galaxy_2.0 as i32 - galaxy.0 as i32).abs() as u32;
            let x_steps: u32 = (galaxy_2.1 as i32 - galaxy.1 as i32).abs() as u32;
            path_sum += y_steps + x_steps;
        }
    }
    println!("Path sum: {}", path_sum);
}
