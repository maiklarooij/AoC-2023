fn main() {
    let grid: Vec<Vec<char>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Find position of animal
    let mut pos: (usize, usize) = (0, 0);
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == 'S' {
                pos = (x, y);
                println!("Found start position: {:?}", pos);
                break;
            }
        }
    }

    let mut dir: char = 'N';
    let mut steps: usize = 0;

    // Follow path
    loop {

        // New direction and position
        dir = new_dir(dir, grid[pos.0][pos.1]);
        match dir {
            'N' => pos.0 -= 1,
            'E' => pos.1 += 1,
            'S' => pos.0 += 1,
            'W' => pos.1 -= 1,
            _ => panic!("Invalid direction: {}", dir),
        }

        steps += 1;
        
        // Check if we are at the end
        if grid[pos.0][pos.1] == 'S' {
            println!("Found postition after {} steps, max distance = {}", steps, steps/2);
            break;
        }
    }

    
}

fn new_dir(dir: char, pipe: char) -> char {
    match pipe {
        '|' => dir,
        '-' => dir,
        'S' => dir,
        '7' => match dir {
            'E' => 'S',
            'N' => 'W',
            _ => panic!("Invalid direction: {}, pipe: {}", dir, pipe),
        }
        'L' => match dir {
            'W' => 'N',
            'S' => 'E',
            _ => panic!("Invalid direction: {}, pipe: {}", dir, pipe),
        }
        'J' => match dir {
            'E' => 'N',
            'S' => 'W',
            _ => panic!("Invalid direction: {}, pipe: {}", dir, pipe),
        }
        'F' => match dir {
            'W' => 'S',
            'N' => 'E',
            _ => panic!("Invalid direction: {}, pipe: {}", dir, pipe),
        }
        _ => panic!("Invalid pipe: {}", pipe),
    }
}