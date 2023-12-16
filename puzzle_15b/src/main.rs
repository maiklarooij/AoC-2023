fn main() {
    let steps = include_str!("../input.txt").split(',').map(|step| step.trim()).collect::<Vec<&str>>();

    // Create 256 boxes
    let mut boxes: Vec<Vec<(String, u32)>> = vec![Vec::new(); 256];

    for step in steps {

        // Calculate the box to go to + retrieve the label
        let mut curr_value: u32 = 0;
        let mut label = Vec::new();
        for c in step.chars() {

            if c.is_alphabetic() {
                curr_value += c as u32;
                curr_value *= 17;
                curr_value %= 256;
                label.push(c);
            }
        }
        let box_to_go = curr_value as usize;
        let label_string = label.iter().cloned().collect::<String>();

        // Remove label from box
        if step.contains('-') {
            let selected_box = &mut boxes[box_to_go];

            for i in 0..selected_box.len() {
                if selected_box[i].0 == label_string {
                    selected_box.remove(i);
                    break;
                }
            }
        }

        // Assign to box
        if step.contains('=') {
            let focal_length = step.split('=').collect::<Vec<&str>>()[1].trim().parse::<u32>().unwrap();
            let selected_box = &mut boxes[box_to_go];

            // Check if label is already in box, replace
            let mut found = false;
            for i in 0..selected_box.len() {
                if selected_box[i].0 == label_string {
                    selected_box[i] = (label_string.clone(), focal_length);
                    found = true;
                    break;
                }
            }

            // If not, add at the end
            if !found {
                selected_box.push((label_string.clone(), focal_length));
            }
        }
    }

    // Score calculation: box nr * label nr * focal length
    let mut final_sum: u32 = 0;
    for (box_i, box_) in boxes.iter().enumerate() {
        for (i, (_label, focal_length)) in box_.iter().enumerate() {
            let score: u32 = (box_i+1) as u32 * (i+1) as u32 * focal_length;
            final_sum += score;
        }
    }

    println!("Final sum: {}", final_sum);
}
