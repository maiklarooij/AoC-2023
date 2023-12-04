fn main() {
    let lines = include_str!("../input.txt").lines();
    let mut total_sum = 0;

    for line in lines {

        // Parsing the line
        let splitted_line = line.split(":").collect::<Vec<&str>>();
        let card_data_splitted = splitted_line[1].split("|").collect::<Vec<&str>>();
        let winning_numbers = numstring_to_vec(card_data_splitted[0]);
        let own_numbers = numstring_to_vec(card_data_splitted[1]);

        // Calculating the points
        let mut points = 0;
        for number in own_numbers {
            if winning_numbers.contains(&number) {
                
                match points {
                    0 => points = 1,
                    _ => points *= 2
                }
            }
        }
        
        total_sum += points;
    }

    println!("Total sum: {}", total_sum);
}

fn numstring_to_vec(numstring: &str) -> Vec<i32> {
    numstring.trim().split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}