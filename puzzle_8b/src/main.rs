use std::collections::HashMap;
use num::integer::lcm;

fn main() {
    let mut lines = include_str!("../input.txt").lines();
    let directions: Vec<char> = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();

    let mut direction_map: HashMap<String, HashMap<char, String>> = HashMap::new();
    let mut nodes_end_A: Vec<String> = Vec::new();

    // Create map of locations and their possible directions
    for line in lines {
        let line_splitted: Vec<&str> = line.split('=').collect();
        let curr_loc: String = line_splitted[0].trim().to_string();
        let dest_locs: Vec<String> = line_splitted[1].split(',').map(|x| x.trim().replace('(', "").replace(')', "")).collect();
        let left_loc: String = dest_locs[0].clone();
        let right_loc: String = dest_locs[1].clone();

        if curr_loc.ends_with('A') {
            nodes_end_A.push(curr_loc.clone());
        }

        let curr_loc_map: HashMap<char, String> = HashMap::from([
            ('L', left_loc),
            ('R', right_loc)
        ]);

        direction_map.insert(curr_loc, curr_loc_map);
    }

    // Follow directions until ZZZ is reached
    let mut zzz_found: bool = false;
    let mut results: Vec<u64> = Vec::new();

    // Find path for every node ending with Z -> it's a loop
    for node in nodes_end_A.clone() {
        let mut curr_node = node.clone();
        let mut curr_steps = 0;
        while !zzz_found {
            for direction in directions.clone() {
                curr_node = direction_map[&curr_node].get(&direction).unwrap().to_string();
                curr_steps += 1;
                if curr_node.ends_with('Z') {
                    zzz_found = true;
                    break;
                }
            }
        }
        results.push(curr_steps);
        zzz_found = false;
    }

    // Use Lowest Common Multiple to find the number of steps
    let final_result: u64 = results.iter().fold(1, |acc, x| lcm(acc, *x));
    println!("Steps: {}", final_result);


}
