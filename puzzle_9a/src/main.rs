fn main() {

    // Parse input
    let histories = include_str!("../input.txt")
        .lines()
        .map(|line| line.split(' ').map(|val| val.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let mut total_history_value: i32 = 0;

    // Iterate over histories
    for history in histories {
        
        let mut current_sequence: Vec<i32> = history.clone();
        let mut first_values: Vec<i32> = Vec::new();

        while !current_sequence.iter().all(|&x| x == 0) {

            // Construct next sequence by the differences
            let mut next_sequence: Vec<i32> = Vec::new();
            for i in 0..current_sequence.len()-1 {
                let diff: i32 = current_sequence[i+1] - current_sequence[i];
                next_sequence.push(diff);
            }

            // Keep track of first values
            first_values.push(current_sequence.first().unwrap().clone());
            current_sequence = next_sequence.clone();
        }
        
        // Calculate total history value
        total_history_value += first_values.iter().rev().fold(0, |acc, x| x - acc);
    }

    println!("Total history value: {}", total_history_value);
}
