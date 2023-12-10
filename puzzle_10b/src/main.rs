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

    let mut dir: char = 'S';
    let mut all_points: Vec<(usize, usize)> = Vec::new();

    // Construct and save path
    loop {

        all_points.push(pos);

        // New direction and position
        dir = new_dir(dir, grid[pos.0][pos.1]);
        match dir {
            'N' => pos.0 -= 1,
            'E' => pos.1 += 1,
            'S' => pos.0 += 1,
            'W' => pos.1 -= 1,
            _ => panic!("Invalid direction: {}", dir),
        }
        
        // Check if we are at the end
        if grid[pos.0][pos.1] == 'S' {
            break;
        }
    }

    // Find enclosed points
    let mut enclosed_points: u32 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {

            // Check if point is already in path
            if all_points.contains(&(y, x)) {
                continue;
            }
            
            // Check the number of passes through the polygon
            let mut nr_passes: u16 = 0;
            for y_c in 0..y {
                if all_points.contains(&(y_c, x)) &&  ['-', 'L', 'F'].contains(&grid[y_c][x]) {
                    nr_passes += 1;
                }
            }
            
            // If odd number of passes, point is enclosed
            if nr_passes % 2 != 0 {
                println!("Found point: {:?}, passes {}", (y, x), nr_passes);
                enclosed_points += 1;
            }
        }
    }

    println!("Found {} enclosed points", enclosed_points);
    
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