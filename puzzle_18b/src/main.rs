fn main() {
    let lines = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    let hexa_digits = lines.into_iter()
        .map(|line| line.split(' ').collect::<Vec<&str>>()[2].replace(")", "").replace("(", "").replace("#", ""))
        .collect::<Vec<String>>();

    // Keep track of position, number of steps and area
    let mut x_pos: i64 = 0;
    let mut y_pos: i64 = 0;
    let mut area: i64 = 0;
    let mut b = 0;

    for dig in hexa_digits {

        // Extract instruction and value
        let first_5 = dig.chars().take(5).collect::<String>();
        let instruction = match dig.chars().nth(5).unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => panic!("Unknown instruction: {}", dig.chars().nth(5).unwrap())
        };
        let value =  i64::from_str_radix(&first_5, 16).unwrap();
        
        // Update position after *value* steps
        let mut new_x_pos = x_pos;
        let mut new_y_pos = y_pos;
        match instruction {
            'R' => new_x_pos += value as i64,
            'L' => new_x_pos -= value as i64,
            'U' => new_y_pos += value as i64,
            'D' => new_y_pos -= value as i64,
            _ => panic!("Unknown instruction: {}", instruction)
        }

        // Shoelace formula for area calculation
        area += ((x_pos * new_y_pos) - (new_x_pos * y_pos)) as i64;

        x_pos = new_x_pos;
        y_pos = new_y_pos;
        b += value;
    }

    // Total points inside polygon + total points on polygon -> Pick's theorem
    let total_points = (area.abs() / 2) - (b/2) + 1 + b;

    println!("Total inner points: {}", total_points);
    
}
