fn main() {
    let lines = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    let instructions = lines.iter()
        .map(|line| (line.split(' ').collect::<Vec<&str>>()[0].chars().next().unwrap(), 
                     line.split(' ').collect::<Vec<&str>>()[1]))
        .collect::<Vec<(char, &str)>>();

    let mut coordinates: Vec<(isize, isize)> = Vec::new();
    let mut x_pos: isize = 0;
    let mut y_pos: isize = 0;
    coordinates.push((x_pos, y_pos));

    // Find all coordinates
    for (instruction, value) in instructions {

        let number_of_steps = value.parse::<isize>().unwrap();
        for _i in 0..number_of_steps {
            match instruction {
                'R' => x_pos += 1,
                'L' => x_pos -= 1,
                'U' => y_pos += 1,
                'D' => y_pos -= 1,
                _ => panic!("Unknown instruction: {}", instruction)
            }
            if !coordinates.contains(&(x_pos, y_pos)) {
                coordinates.push((x_pos, y_pos));
            }
        }
    }

    // Shoelace formula for area calculation
    let mut area = 0;

    for i in 0..coordinates.len() {
        let x1 = coordinates[i].0;
        let y1 = coordinates[i].1;
        let x2 = coordinates[(i + 1) % coordinates.len()].0;
        let y2 = coordinates[(i + 1) % coordinates.len()].1;

        area += (x1 * y2) - (x2 * y1);
    }

    // Total points inside polygon + total points on polygon -> Pick's theorem
    let b = coordinates.len() as isize;
    let total_points = (area.abs() / 2) - (b/2) + 1 + b;

    println!("Total inner points: {}", total_points);
    
}
