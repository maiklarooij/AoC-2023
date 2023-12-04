fn main() {
    let lines = include_str!("../input.txt").lines();
    let mut total_matches: Vec<u32> = Vec::new();

    // First get for each card the number of matches
    for line in lines {

        // Parsing the line
        let splitted_line = line.split(":").collect::<Vec<&str>>();
        let card_data_splitted = splitted_line[1].split("|").collect::<Vec<&str>>();
        let winning_numbers = numstring_to_vec(card_data_splitted[0]);
        let own_numbers = numstring_to_vec(card_data_splitted[1]);

        // Calculating the points
        let mut num_matching = 0;
        for number in own_numbers {
            if winning_numbers.contains(&number) {
                num_matching += 1;
            }
        }

        total_matches.push(num_matching);
    }

    // Then, for each card, calculate the number of copies it gives
    // Start with 1 copy for every card
    let mut total_with_copies: Vec<u32> = vec![1; total_matches.len()];
    for i in 0..total_matches.len() {

        let nr_matching = total_matches[i] as usize;
        let nr_copies = total_with_copies[i];

        for j in 1..(nr_matching + 1) {
            if i + j >= total_matches.len() {
                break;
            }
            total_with_copies[i + j] += nr_copies;
        }
    }

    println!("Total sum: {}", total_with_copies.iter().sum::<u32>());
}

fn numstring_to_vec(numstring: &str) -> Vec<i32> {
    numstring.trim().split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}