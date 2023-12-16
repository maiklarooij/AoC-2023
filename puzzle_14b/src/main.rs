fn main() {
    let mut rocks = include_str!("../input.txt").lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    
    // Keep track of seen states to find cycles
    let mut seen_states: Vec<Vec<Vec<char>>> = Vec::new();
    seen_states.push(rocks.clone());
    let mut cycles: usize = 0;
    let mut found_cycle: bool = false;

    // Keep going for cycles!
    while cycles < 1000000000 {

        // Rotate and move rocks
        for _i in 0..4 {
            let new_rocks: Vec<Vec<char>> = move_rocks(&rocks);
            rocks = rotate_rocks(&new_rocks);
        }

        cycles += 1;
        
        // We have not found a cycle yet
        if !found_cycle {

            // Check if we have seen this state before
            let index = check_same(&rocks, &seen_states);

            // If we have seen this state before, we have found a cycle
            if index != 0 {
                found_cycle = true;
                
                // Calculate the cycle length and start
                let cycle_length = cycles - (index);
                let cycle_start = index;
                
                // Calculate the start of the cycle to skip to
                let mut start_at = 1000000000 - cycle_start;
                while start_at % cycle_length != 0 {
                    start_at -= 1;
                }
                
                // Skip to the start of the last possible cycle
                cycles = start_at+cycle_start;
                
            }

            // Add the current state to the seen states
            seen_states.push(rocks.clone());
        }
    }

    println!("Total load: {}", calculate_load(&rocks));

}

fn calculate_load(rocks: &Vec<Vec<char>>) -> usize {
    let nr_cols = rocks[0].len();
    let nr_rows = rocks.len();
    let mut total_load = 0;

    // Calculate the total load -> nr of rows under rock
    for i in 0..nr_rows {
        for j in 0..nr_cols {
            match rocks[i][j] {
                'O' => total_load += nr_rows - i,
                _ => continue
            }
        }
    
    }
    total_load
}

fn check_same(rocks: &Vec<Vec<char>>, seen_states: &Vec<Vec<Vec<char>>>) -> usize {
    let nr_cols = rocks[0].len();
    let nr_rows = rocks.len();

    // Check if we have seen this state before, return index of state
    for (i, state) in seen_states.iter().enumerate() {
        let mut state_seen = true;
        for i in 0..nr_cols {
            for j in 0..nr_rows {
                if rocks[i][j] != state[i][j] {
                    state_seen = false;
                }
            }
        }
        if state_seen {
            return i as usize;
        }
    }

    // Not seen
    0
}

fn rotate_rocks(rocks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Rotate the grid 90 degrees clockwise
    let nr_cols = rocks[0].len();
    let nr_rows = rocks.len();
    let mut new_rocks = rocks.clone();
    let mut new_rocks_trans = rocks.clone();

    // Transpose
    for i in 0..nr_cols {
        for j in 0..nr_rows {
            new_rocks_trans[i][j] = rocks[j][i];
        }
    }

    // Reverse rows
    for i in 0..nr_rows {
        let reversed_row = new_rocks_trans[i].clone().into_iter().rev().collect::<Vec<char>>();
        new_rocks[i] = reversed_row;
    }

    new_rocks
}

fn move_rocks(rocks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let nr_cols = rocks[0].len();
    let nr_rows = rocks.len();
    let mut new_rocks = rocks.clone();
    for i in 0..nr_cols {
        
        // Tilted north: keep track of the last cube and the number of rounded cubes at the moment
        let mut last_cube = 0;
        let mut nr_rounded = 0;
        for j in 0..nr_rows {
            
            match rocks[j][i] {

                // Keep track of last cube where rocks will fall onto
                '#' => {
                    last_cube = j+1;
                    nr_rounded = 0;
                }
                '.' => continue,

                // Calculate the load for the current cube: last steady rock - round stones before
                'O' => {
                    new_rocks[j][i] = '.';
                    new_rocks[last_cube+nr_rounded][i] = 'O';
                    nr_rounded += 1;
                }
                _ => panic!("Invalid character")
            }
        }
    }
    new_rocks
}