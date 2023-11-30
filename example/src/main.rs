fn main() {
    let lines = include_str!("../input.txt").lines()
    .map(|line| (line.split_once(" ").unwrap().0, line.split_once(" ").unwrap().1.parse::<i8>().unwrap()));

    let mut depth: i32 = 0;
    let mut x: i32 = 0;
    let mut aim: i32 = 0;

    for (movement, value) in lines {
        match (movement, value) {
            ("forward", value) => {
                x += value as i32;
                depth += aim * value as i32;
            } 
            ("down", value) => aim += value as i32,
            ("up", value) => aim -= value as i32,
            _ => panic!("Invalid movement")
        }
    }

    println!("Depth: {}", depth);
    println!("X: {}", x);
    println!("Result: {}", depth * x);
}
