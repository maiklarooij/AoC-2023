fn main() {
    let steps = include_str!("../input.txt").split(',').map(|step| step.trim()).collect::<Vec<&str>>();

    let mut final_sum: u32 = 0;
    for step in steps {

        let mut curr_value: u32 = 0;
        for c in step.chars() {
            curr_value += c as u32;
            curr_value *= 17;
            curr_value %= 256;
        }
        final_sum += curr_value;
    }

    println!("Final sum: {}", final_sum);
}
