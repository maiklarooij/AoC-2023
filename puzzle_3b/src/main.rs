use std::collections::HashMap;

fn main() {
    let grid: Vec<Vec<char>> = include_str!("../input.txt").lines().map(|line| line.chars().collect()).collect();
    let n: usize = grid.len();

    let mut curr_num: String = String::new();
    let mut curr_valid: bool = false;
    let mut total_sum: u32 = 0;
    let mut gear_loc: (usize, usize) = (0, 0);

    let mut gear_map: HashMap<(usize, usize), String> = HashMap::new();

    for row in 0..n {
        for col in 0..n {

            let c = grid[row][col];
            
            if c.is_numeric() {
                curr_num = format!("{}{}", curr_num, c);
                
                // Check neighbors
                for row_offset in [-1, 0, 1] {
                    for col_offset in [-1, 0, 1] {
                        let row_to_check: usize = (row as isize + row_offset as isize) as usize;
                        let col_to_check: usize = (col as isize + col_offset as isize) as usize;
                        
                        // Out of bounds / overflow
                        if row_to_check >= n || col_to_check >= n {
                            continue;
                        }
                        
                        // Check if the current cell is a symbol
                        let to_check = grid[row_to_check][col_to_check];
                        if to_check == '*' {
                            curr_valid = true;
                            gear_loc = (row_to_check, col_to_check);
                        }
                    }
                }
            }

            // Check total found number
            if !c.is_numeric() || col == n - 1  {
                if curr_valid {
                    if gear_map.contains_key(&gear_loc) {
                        let gear_part_1: u32 = gear_map.get(&gear_loc).unwrap().parse().unwrap();
                        let gear_part_2: u32 = curr_num.parse().unwrap();
                        total_sum += gear_part_1 * gear_part_2;

                        println!("{} * {} = {}", gear_part_1, gear_part_2, gear_part_1 * gear_part_2);

                        gear_map.remove(&gear_loc);
                    }
                    else {
                        gear_map.insert(gear_loc, curr_num);
                    }
                }

                curr_num = String::new();
                curr_valid = false;
            }
        }
    }

    println!("Total sum: {}", total_sum);
    
}   
