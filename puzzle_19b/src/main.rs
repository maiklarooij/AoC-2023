use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("../input.txt").lines();
    
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();

    // Parse rules
    while let Some(line) = lines.next() {
        
        if line == "" {
            break;
        }

        let rule_name = line.split("{").collect::<Vec<&str>>()[0].trim().to_string();
        let rule = line.split("{").collect::<Vec<&str>>()[1].split(',').map(|s| s.trim().replace('}', "").to_string()).collect::<Vec<String>>();
        rules.insert(rule_name, rule);
    }

    // Get number of combinations by checking all rules from "in" and narrowing down the ranges
    let combinations = nr_combinations(&rules, (1, 4000), (1, 4000), (1, 4000), (1, 4000), "in");

    println!("Number of combinations: {}", combinations);
}

fn get_new_range(old_range: (usize, usize), sign: char, val_to_check: usize) -> ((usize, usize), (usize, usize)) {

    // Returns the range if true and range if false
    match sign {
        '>' => {
            if old_range.0 < val_to_check {
                return ((val_to_check + 1, old_range.1), (old_range.0, val_to_check));
            }
            else {
                // false range gets set to (0, 0) because the value to set is not in the range
                return (old_range, (0, 0));
            }
        },
        '<' => {
            if old_range.1 > val_to_check {
                return ((old_range.0, val_to_check - 1), (val_to_check, old_range.1));
            }
            else {
                // false range gets set to (0, 0) because the value to set is not in the range
                return (old_range, (0, 0));
            }
        },
        _ => panic!("Invalid sign")
    }
}

fn nr_combinations(rules: &HashMap<String, Vec<String>>, mut x_range: (usize, usize), 
                   mut m_range: (usize, usize), mut a_range: (usize, usize), mut s_range: (usize, usize),
                   current_rule: &str) -> u64 {
                
    // Can't have any combinations if any of the ranges are 0
    if x_range == (0, 0) || m_range == (0, 0) || a_range == (0, 0) || s_range == (0, 0) {
        return 0;
    }

    let mut found_combinations: u64 = 0;

    // If we have found the answer, return the number of combinations
    if current_rule == "A" {
        found_combinations += ((x_range.1 - x_range.0 + 1) * (m_range.1 - m_range.0 + 1) * (a_range.1 - a_range.0 + 1) * (s_range.1 - s_range.0 + 1)) as u64;
        return found_combinations;
    }
    else if current_rule == "R" {
        return found_combinations;
    }
    
    let rules_to_check = rules.get(current_rule).unwrap();
    for rule_part in rules_to_check {

        // If we have found the answer, return the number of combinations
        if rule_part == "A" {
            found_combinations += ((x_range.1 - x_range.0 + 1) * (m_range.1 - m_range.0 + 1) * (a_range.1 - a_range.0 + 1) * (s_range.1 - s_range.0 + 1)) as u64;
            return found_combinations;
        }
        else if rule_part == "R" {
            return found_combinations;
        }
        
        // Apply rule, split ranges and recursively call nr_combinations
        if rule_part.contains('>') || rule_part.contains('<') {
            for sign in ['>', '<'] {
                if rule_part.contains(sign) {

                    // Parse rule further
                    let var_to_check = rule_part.split(sign).collect::<Vec<&str>>()[0];
                    let val_to_check = rule_part.split(sign).collect::<Vec<&str>>()[1].split(':').collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
                    let answer = rule_part.split(sign).collect::<Vec<&str>>()[1].split(':').collect::<Vec<&str>>()[1];
                    
                    // Find nr of combinations for new ranges if the rule is true, 
                    // update the range if the rule is false and go on to the next part
                    match var_to_check {
                        "x" => {
                            let (true_range, false_range) = get_new_range(x_range, sign, val_to_check as usize);
                            found_combinations += nr_combinations(rules, true_range, m_range, a_range, s_range, answer);
                            x_range = false_range;
                        },
                        "m" => {
                            let (true_range, false_range) = get_new_range(m_range, sign, val_to_check as usize);
                            found_combinations += nr_combinations(rules, x_range, true_range, a_range, s_range, answer);
                            m_range = false_range;
                        },
                        "a" => {
                            let (true_range, false_range) = get_new_range(a_range, sign, val_to_check as usize);
                            found_combinations += nr_combinations(rules, x_range, m_range, true_range, s_range, answer);
                            a_range = false_range;
                        },
                        "s" => {
                            let (true_range, false_range) = get_new_range(s_range, sign, val_to_check as usize);
                            found_combinations += nr_combinations(rules, x_range, m_range, a_range, true_range, answer);
                            s_range = false_range;
                        }
                        _ => panic!("Invalid variable")
                    }
                }
            }
        }
        // No 'A' or 'R', no '>' or '<', so we have a rule that indicates a new rule to check
        else {
            found_combinations += nr_combinations(rules, x_range, m_range, a_range, s_range, rule_part);
        }
    }

    return found_combinations; 
}
