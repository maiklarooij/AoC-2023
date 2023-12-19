use std::collections::HashMap;

fn main() {
    let grid = include_str!("../input.txt").lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut highest_score = 0;
    
    for j in [0, grid[0].len()-1] {

        for i in 0..grid.len() {
            
            // Check first and last column
            {
                let mut visited: &mut Vec<Vec<HashMap<char, bool>>> = &mut vec![vec![
                    HashMap::from(
                        [('R', false), ('L', false), ('U', false), ('D', false)])
                    ; grid[0].len()]; grid.len()];
                let start_pos = (i as i32, j as i32);
                let start_dir = if j == 0 { 'R' } else { 'L' };
                follow_beam(&grid, start_pos, start_dir, &mut visited);

                let final_score = final_score(&grid, &visited);
                if final_score > highest_score {
                    highest_score = final_score;
                }
            }
            // Check first and last row
            {
                let mut visited: &mut Vec<Vec<HashMap<char, bool>>> = &mut vec![vec![
                    HashMap::from(
                        [('R', false), ('L', false), ('U', false), ('D', false)])
                    ; grid[0].len()]; grid.len()];
                let start_pos = (j as i32, i as i32);
                let start_dir = if j == 0 { 'D' } else { 'U' };
                follow_beam(&grid, start_pos, start_dir, &mut visited);

                let final_score = final_score(&grid, &visited);
                if final_score > highest_score {
                    highest_score = final_score;
                }
            }
        }
    }

    println!("Energized: {}", highest_score);

}

fn final_score(grid: &Vec<Vec<char>>, visited: &Vec<Vec<HashMap<char, bool>>>) -> u32 {
    let mut total_score = 0;
    for row in visited.clone() {
        for col in row {
            if col.iter().filter(|&(_, v)| *v).count() > 0 {
                total_score += 1;
            }
            else {
            }
        }
    }
    total_score
}

fn move_forward(position: (i32, i32), dir: char) -> (i32, i32) {
    match dir {
        'R' => (position.0, position.1+1),
        'L' => (position.0, position.1-1),
        'U' => (position.0-1, position.1),
        'D' => (position.0+1, position.1),
        _ => panic!("Unknown direction: {}", dir)
    }
}

fn out_of_bounds(grid: &Vec<Vec<char>>, position: (i32, i32)) -> bool {
    position.0 < 0 || position.1 < 0 || 
    position.0 >= grid.len() as i32 || 
    position.1 >= grid[0].len() as i32
}

fn follow_beam(grid: &Vec<Vec<char>>, position: (i32, i32), dir: char, mut visited: &mut Vec<Vec<HashMap<char, bool>>>) -> u32 {
    
    let mut current_pos = position;
    let mut current_dir = dir;

    let mut energized = 0;

    loop {

        visited[current_pos.0 as usize][current_pos.1 as usize].insert(current_dir, true);

        match grid[current_pos.0 as usize][current_pos.1 as usize] {
            '.' => {
                current_pos = move_forward(current_pos, current_dir);
                if out_of_bounds(grid, current_pos) {
                    return energized;
                }
                if visited[current_pos.0 as usize][current_pos.1 as usize][&current_dir] == true {
                    return energized;
                }
            },
            '\u{005C}' => {
                match current_dir {
                    'R' => current_dir = 'D',
                    'L' => current_dir = 'U',
                    'U' => current_dir = 'L',
                    'D' => current_dir = 'R',
                    _ => panic!("Unknown direction: {}", current_dir)
                }
                println!("{}: {}", current_pos.0, current_pos.1);
                current_pos = move_forward(current_pos, current_dir);
                if out_of_bounds(grid, current_pos) {
                    return energized;
                }
                if visited[current_pos.0 as usize][current_pos.1 as usize][&current_dir] == true {
                    return energized;
                }
            },
            '/' => {
                match current_dir {
                    'R' => current_dir = 'U',
                    'L' => current_dir = 'D',
                    'U' => current_dir = 'R',
                    'D' => current_dir = 'L',
                    _ => panic!("Unknown direction: {}", current_dir)
                }
                current_pos = move_forward(current_pos, current_dir);
                if out_of_bounds(grid, current_pos) {
                    return energized;
                }
                if visited[current_pos.0 as usize][current_pos.1 as usize][&current_dir] == true {
                    return energized;
                }
            },
            '-' => {
                if current_dir == 'R' || current_dir == 'L' {
                    current_pos = move_forward(current_pos, current_dir);
                    if out_of_bounds(grid, current_pos) {
                        return energized;
                    }
                    if visited[current_pos.0 as usize][current_pos.1 as usize][&current_dir] == true {
                        return energized;
                    }
                }
                else {
                    let new_pos_l = move_forward(current_pos, 'L');
                    if !out_of_bounds(grid, new_pos_l) {
                        follow_beam(grid, new_pos_l, 'L', &mut visited);
                    }
                    current_pos = move_forward(current_pos, 'R');
                    if out_of_bounds(grid, current_pos) {
                        return energized;
                    }
                    current_dir = 'R';
                    if visited[current_pos.0 as usize][current_pos.1 as usize][&current_dir] == true {
                        return energized;
                    }
                }
            },
            '|' => {
                if current_dir == 'U' || current_dir == 'D' {
                    current_pos = move_forward(current_pos, current_dir);
                    if out_of_bounds(grid, current_pos) {
                        return energized;
                    }
                    if visited[current_pos.0 as usize][current_pos.1 as usize][&current_dir] == true {
                        return energized;
                    }
                }
                else {
                    let new_pos_u = move_forward(current_pos, 'U');
                    if !out_of_bounds(grid, new_pos_u) {
                        follow_beam(grid, new_pos_u, 'U', &mut visited);
                    }
                    current_pos = move_forward(current_pos, 'D');
                    if out_of_bounds(grid, current_pos) {
                        return energized;
                    }
                    current_dir = 'D';
                    if visited[current_pos.0 as usize][current_pos.1 as usize][&current_dir] == true {
                        return energized;
                    }
                }
            },
            _ => panic!("Unknown character: {}", grid[current_pos.0 as usize][current_pos.1 as usize])
        }
    }

    energized

}
