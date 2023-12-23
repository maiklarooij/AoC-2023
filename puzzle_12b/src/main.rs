use std::collections::HashMap;

fn main() {
    let lines = include_str!("../input.txt")
        .lines();

    let mut sum_valid: u64 = 0;
    for line in lines {

        let splitted_line = line.split(" ").collect::<Vec<&str>>();

        // Get springs and repeat 5 times
        let mut springs = splitted_line[0].chars().collect::<Vec<char>>();
        springs.push('?');
        let repeated_springs = springs.clone().into_iter().cycle().take(springs.len() * 5).collect::<Vec<char>>();

        // Get spring sizes and repeat 5 times
        let mut spring_sizes = splitted_line[1].split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        spring_sizes = spring_sizes.clone().into_iter().cycle().take(spring_sizes.len() * 5).collect::<Vec<u32>>();
        
        // Utilize cache to speed up
        let cache: &mut HashMap<(usize, usize), u64> = &mut HashMap::new();
        
        // Find all valid arrangements
        let valid_arrangements_springs: u64 = arrangements(&repeated_springs, &spring_sizes, 0, 0, cache);
        sum_valid += valid_arrangements_springs;
    }

    println!("Sum valid: {}", sum_valid);
}

fn arrangements(springs: &Vec<char>, spring_sizes: &Vec<u32>, spring_i: usize, size_i: usize, mut cache: &mut HashMap<(usize, usize), u64>) -> u64 {
    
    // Check cache if solution already exists
    if cache.contains_key(&(spring_i, size_i)) {
        return cache.get(&(spring_i, size_i)).unwrap().clone();
    }

    // We reach end of springs, check if all sizes are used
    if spring_i >= springs.len() {
        // Valid arrangement!
        if size_i == spring_sizes.len() {
            return 1;
        }
        return 0;
    }
    
    // Operational spring: skip and continue
    if springs[spring_i] == '.' {
        let total: u64 = arrangements(springs, spring_sizes, spring_i+1, size_i, &mut cache);
        cache.insert((spring_i, size_i), total);
        return total;
    }

    // Other spring: check if current group is valid
    let mut total: u64 = 0;
    if size_i < spring_sizes.len() {

        // Check current group
        let mut next_spring_id: usize  = spring_i;
        let mut count: u32 = 0;

        // Go on until next spring is broken or group is complete
        while springs[next_spring_id] != '.' &&
            !(springs[next_spring_id] == '?' && count == spring_sizes[size_i]) &&
            count <= spring_sizes[size_i] {
            count += 1;
            next_spring_id += 1;

            if next_spring_id >= springs.len() {
                break;
            }
        }

        // Check if current group is valid
        if count == spring_sizes[size_i] {
            
            if spring_i + (count as usize) < springs.len() {
               
                // If next spring is not broken, go to next group and leave a gap
                if springs[spring_i + count as usize] != '#' {
                    total += arrangements(springs, spring_sizes, spring_i+(count as usize)+1, size_i+1, cache);
                }
                // If next spring is broken, go to next group without leaving a gap
                else {
                    total += arrangements(springs, spring_sizes, spring_i+(count as usize), size_i+1, cache);
                }
            }
        }
    }

    // Also check next spring if it is a '?'
    if springs[spring_i] == '?' {
        total += arrangements(springs, spring_sizes, spring_i+1, size_i, cache);
    }

    cache.insert((spring_i, size_i), total);

    return total;
}
