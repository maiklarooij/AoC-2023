// Day 1: Trebuchet?!
// https://adventofcode.com/2023/day/1

fn main() {

    let lines = include_str!("../input.txt").lines();

    // Patterns to match
    let patterns: [&str; 18] = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
        ];
    let mut sum: u32 = 0;

    for line in lines {

        // Keep track of first and last number in line
        let mut first_hit: (usize, &str) = (usize::MAX, "None");
        let mut last_hit: (usize, &str) = (0, "None");
        
        // Find first and last number in line based on patterns
        for pattern in patterns {
            let first_pat_hit: Option<usize> = line.find(pattern);
            let last_pat_hit: Option<usize> = line.rfind(pattern);

            if first_pat_hit.is_some() {
                let first_hit_index: usize = first_pat_hit.unwrap();
                if first_hit_index <= first_hit.0 {
                    first_hit = (first_hit_index, pattern);
                }
            }

            if last_pat_hit.is_some() {
                let last_hit_index: usize = last_pat_hit.unwrap();
                if last_hit_index >= last_hit.0 {
                    last_hit = (last_hit_index, pattern);
                }
            }
        }

        // Combine first and last number in line
        let combined: u32 = format!(
            "{}{}", translate_to_int(first_hit.1), 
                    translate_to_int(last_hit.1)
                ).parse::<u32>().unwrap();

        sum += combined;
    }

    println!("Calibration sum: {}", sum);
}

fn translate_to_int(text: &str) -> u32 {
    match text {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => text.parse().unwrap()
    }
}
