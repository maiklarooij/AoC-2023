fn main() {
    
    // Parse input into times and distances
    let mut lines = include_str!("../input.txt").lines();
    let time: u64 = extract_numbers(lines.next().unwrap());
    let distance: u64 = extract_numbers(lines.next().unwrap());

    let mut start_win_time: u64 = 0;
    let mut end_win_time: u64 = 0;
    
    // Find start of winning range
    for i in 0..time+1 {

        if winning_time(time, distance, i) {
            start_win_time = i;
            break;
        }
    }

    // Find end of winning range
    for i in 0..time+1 {
        let actual_i = time - i;

        if winning_time(time, distance, actual_i) {
            end_win_time = actual_i;
            break;
        }

    }

    println!("Start: {}, End: {}, Total: {}", start_win_time, end_win_time, (end_win_time - start_win_time) + 1);    
}

fn extract_numbers(line: &str) -> u64 {
    line.split(':').collect::<Vec<&str>>()[1].trim().replace(" ", "").parse().unwrap()
}

fn winning_time(time: u64, distance: u64, hold_time: u64) -> bool {
    let time_to_race = time - hold_time;
    let total_distance = hold_time * time_to_race;

    if total_distance > distance {
        return true;
    }
    return false;
}
