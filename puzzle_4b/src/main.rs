fn main() {
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();
    let n_lines: usize = lines.len();
    let mut total_with_copies: Vec<u32> = vec![1; n_lines];

    // First get for each card the number of matches
    for i in 0..n_lines {

        let line: &str = lines[i];

        // Parsing the line
        let splitted_line = line.split(":").collect::<Vec<&str>>();
        let card_data_splitted = splitted_line[1].split("|").collect::<Vec<&str>>();
        let winning_numbers = numstring_to_vec(card_data_splitted[0]);
        let own_numbers = numstring_to_vec(card_data_splitted[1]);

        // Calculating the points
        let mut num_matching: usize = 0;
        for number in own_numbers {
            if winning_numbers.contains(&number) {
                num_matching += 1;
            }
        }

        let nr_copies: u32 = total_with_copies[i];
        for j in 1..(num_matching + 1) {
            if i + j >= n_lines {
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