fn main() {
    let mut lines = include_str!("../input.txt").lines();

    // Extract seeds
    let seeds: Vec<u64> = lines.next().unwrap().split(':')
        .collect::<Vec<&str>>()[1]
        .trim().split(' ')
        .map(|s| s.parse().unwrap()).collect();

    // Parse whole almanac into vector of stages
    // Each stage is a vector of tuples (dest, source, range)
    let mut curr_vec: Vec<(u64, u64, u64)> = Vec::new();
    let mut almanac: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    lines.next();
    for line in lines {
        
        // If line is empty, push current stage and start new one
        if line == "\n" || line == "" {
            almanac.push(curr_vec);
            curr_vec = Vec::new();
            continue;
        }
        if !line.chars().nth(0).unwrap().is_numeric() {
            continue;
        }

        // Extract values from line
        let splitted_line = line.split(' ').collect::<Vec<&str>>();
        let curr = (
            splitted_line[0].parse().unwrap(), 
            splitted_line[1].parse().unwrap(),
            splitted_line[2].parse().unwrap());
        curr_vec.push(curr);
    }
    almanac.push(curr_vec);

    // Go through stages for every seed and find min
    let mut min_end: u64 = u64::MAX;
    for seed in seeds {

        let mut curr_number = seed;
        for stage in &almanac {
            
            for (dest, source, range) in stage {
                
                // If current number is in range, transform it
                if curr_number >= *source && curr_number < *source + *range {
                    curr_number = *dest + (curr_number - *source);
                    break;
                }
            }
        }

        if curr_number < min_end {
            min_end = curr_number;
        }
    }

    println!("Min: {}", min_end);
}
