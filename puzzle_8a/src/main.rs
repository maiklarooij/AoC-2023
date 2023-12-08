use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("../input.txt").lines();
    let directions: Vec<char> = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();

    let mut direction_map: HashMap<String, HashMap<char, String>> = HashMap::new();

    // Create map of locations and their possible directions
    for line in lines {
        let line_splitted: Vec<&str> = line.split('=').collect();
        let curr_loc: String = line_splitted[0].trim().to_string();
        let dest_locs: Vec<String> = line_splitted[1].split(',').map(|x| x.trim().replace('(', "").replace(')', "")).collect();
        let left_loc: String = dest_locs[0].clone();
        let right_loc: String = dest_locs[1].clone();

        let curr_loc_map: HashMap<char, String> = HashMap::from([
            ('L', left_loc),
            ('R', right_loc)
        ]);

        direction_map.insert(curr_loc, curr_loc_map);
    }

    // Follow directions until ZZZ is reached
    let mut player_at: &str = "AAA";
    let mut zzz_found: bool = false;
    let mut steps: u32 = 0;
    while !zzz_found {
        for direction in directions.clone() {
            player_at = direction_map[player_at].get(&direction).unwrap();
            steps += 1;
            if player_at == "ZZZ" {
                zzz_found = true;
                break;
            }
        }
    }

    println!("Steps: {}", steps);


}
