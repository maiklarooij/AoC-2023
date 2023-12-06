fn main() {
    
    // Parse input into times and distances
    let mut lines = include_str!("../input.txt").lines();
    let times: Vec<u32> = extract_numbers(lines.next().unwrap());
    let distances: Vec<u32> = extract_numbers(lines.next().unwrap());

    let mut total_score: u32 = 1;

    // Check each race
    for i in 0..times.len() {

        let mut times_won: u32 = 0;

        // Check each option
        for option in 0..times[i]+1 {
            
            if winning_time(times[i], distances[i], option) {
                times_won += 1;
            }
        }

        total_score *= times_won;
    }

    println!("Total score: {}", total_score);
    
}

fn extract_numbers(line: &str) -> Vec<u32> {
    line.split(' ').filter_map(|x| x.parse::<u32>().ok()).collect::<Vec<u32>>()
}

fn winning_time(time: u32, distance: u32, hold_time: u32) -> bool {
    let time_to_race = time - hold_time;
    let total_distance = hold_time * time_to_race;

    if total_distance > distance {
        return true;
    }
    return false;
}