use itertools::Itertools;

fn main() {
    let grid = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start_pos: (usize, usize) = find_start_pos(&grid);
    let mut all_positions: Vec<(usize, usize)> = vec![start_pos];

    // Find all positions after 64 steps
    for _i in 0..64 {
        let mut new_found_positions: Vec<(usize, usize)> = Vec::new();

        // For every positions, find possible steps
        for pos in &all_positions {
            let possible_steps: Vec<(usize, usize)> = get_possible_steps(&grid, pos);
            new_found_positions.extend(possible_steps);
        }

        // All unique positions
        all_positions = new_found_positions.into_iter().unique().collect();
    }

    println!("Number of positions: {}", all_positions.len());
}

fn find_start_pos(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                return (y, x);
            }
        }
    }
    panic!("No start position found!");
}

fn get_possible_steps(grid: &Vec<Vec<char>>, pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut possible_steps: Vec<(usize, usize)> = Vec::new();

    for sign1 in [-1 as i32, 1 as i32] {
        for x_or_y in ['x', 'y'] {
            
            // Calculate next position
            // Converting needed because usize can't be negative
            let grid_len: i32 = grid.len() as i32;
            let grid_len2: i32 = grid[0].len() as i32;
            let (y, x): (i32, i32) = (pos.0 as i32, pos.1 as i32);
            let (next_y, next_x): (i32, i32) = match x_or_y {
                'y' => (y + sign1, x),
                'x' => (y, x + sign1),
                _ => panic!("Invalid x_or_y: {}", x_or_y),
            };

            // Check out of bounds
            if next_y < 0 || next_y >= grid_len {
                continue;
            }
            if next_x < 0 || next_x >= grid_len2 {
                continue;
            }

            // Convert back and check not rock
            let (new_y, new_x): (usize, usize) = (next_y as usize, next_x as usize);
            if grid[new_y][new_x] != '#' {
                possible_steps.push((new_y, new_x));
            }
        }
    }
    possible_steps
}
