fn main() {
    let lines = include_str!("../input.txt").lines();
    let mut patterns: Vec<Vec<Vec<char>>> = Vec::new();
    let mut current_pattern: Vec<Vec<char>> = Vec::new();

    // Parse patterns into workable format
    for line in lines {

        if line == "" {
            patterns.push(current_pattern);
            current_pattern = Vec::new();
            continue;
        }

        let current_line = line.chars().collect::<Vec<char>>();
        current_pattern.push(current_line);
    }
    patterns.push(current_pattern);

    let mut notes: u32 = 0;
    for pattern in patterns {
        let pattern_y: usize = pattern.len();
        let pattern_x: usize = pattern[0].len();
        
        // Check rows
        for i in 0..pattern_y-1 {
            let valid = check_validity(pattern.clone(), i);
            if valid {
                notes += ((i as u32) + 1) * 100;
                break;
            }
        }

        // Check columns
        let flipped_pattern = flip_pattern(pattern.clone());
        for i in 0..pattern_x-1 {
            let valid = check_validity(flipped_pattern.clone(), i);
            if valid {
                notes += (i as u32) + 1;
                break;
            }
        }

    }

    println!("Notes: {}", notes);
}

fn count_smudges(list_1: Vec<char>, list_2: Vec<char>) -> u32 {
    let mut count: u32 = 0;
    for i in 0..list_1.len() {
        if list_1[i] != list_2[i] {
            count += 1;
        }
    }
    count
}

fn flip_pattern(pattern: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Makes rows columns and columns rows ;)
    let mut flipped_pattern: Vec<Vec<char>> = Vec::new();
    
    for i in 0..pattern[0].len() {
        let mut current_col = Vec::new();
        for row in &pattern {
            current_col.push(row[i]);
        }
        flipped_pattern.push(current_col);
    }
    flipped_pattern
}

fn check_validity(pattern: Vec<Vec<char>>, i: usize) -> bool {
    let pattern_y: usize = pattern.len();
    if pattern[i] == pattern[i+1] || count_smudges(pattern[i].clone(), pattern[i+1].clone()) == 1 {
        let mut check_i: isize = (i as isize)-1;
        let mut check_i_other: isize = (i as isize)+2;
        let mut smudges = count_smudges(pattern[i].clone(), pattern[i+1].clone());
        while check_i >= 0 && check_i_other < pattern_y as isize {
            let counted_smudges = count_smudges(pattern[check_i as usize].clone(), pattern[check_i_other as usize].clone());

            if counted_smudges > 0 {
                smudges += counted_smudges;
                if smudges > 1 {
                    return false;
                }
                check_i -= 1;
                check_i_other += 1;
                continue;
            }
            if pattern[check_i as usize] == pattern[check_i_other as usize] {
                check_i -= 1;
                check_i_other += 1;
            } else {
                return false;
            }
        }
        if smudges == 1 {
            return true;
        }
    }
    return false
}
