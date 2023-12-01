// Day 1: Trebuchet?!
// https://adventofcode.com/2023/day/1

fn main() {
    
    // Filter digits -> combine first and last char -> to int -> sum
    let sum: u32 = include_str!("../input.txt").lines()
        .map(
            |line: &str| line.chars().filter(|c| c.is_digit(10)).collect()
        )
        .map(
            |line: Vec<char>| format!("{}{}",
            line.first().unwrap(),
            line.last().unwrap()).parse::<u32>().unwrap()
        )
        .sum();

    println!("Calibration sum: {}", sum)
}
