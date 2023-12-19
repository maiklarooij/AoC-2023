use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("../input.txt").lines();
    
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut workflows: Vec<HashMap<String, u32>> = Vec::new();

    // Parse rules
    while let Some(line) = lines.next() {
        
        if line == "" {
            break;
        }

        let rule_name = line.split("{").collect::<Vec<&str>>()[0].trim().to_string();
        let rule = line.split("{").collect::<Vec<&str>>()[1].split(',').map(|s| s.trim().replace('}', "").to_string()).collect::<Vec<String>>();
        rules.insert(rule_name, rule);
    }
    // Parse workflows
    while let Some(line) = lines.next() {

        let replaced_line = &line.replace("{", "").replace("}", "");
        let vars = replaced_line.split(",").map(|s| (s.split('=').collect::<Vec<&str>>()[0], 
            s.split('=').collect::<Vec<&str>>()[1].parse().unwrap() )).collect::<Vec<(&str, u32)>>();
        
        let mut workflow = HashMap::new();
        for var in vars {
            workflow.insert(var.0.to_string(), var.1);
        }
        workflows.push(workflow);
    }

    let mut accepted_score: u32 = 0;

    for workflow in &workflows {
        let mut current_rule: String = "in".to_string();

        // We have not found the answer yet
        while !(current_rule == "A") && !(current_rule == "R") {

            let rules_to_check = rules.get(&current_rule).unwrap();
            let mut answer_found = false;

            // Go through rules until we find the next rule/answer
            for rule_part in rules_to_check {

                for sign in ['>', '<'] {
                    if rule_part.contains(sign) {

                        // Parse rule further
                        let var_to_check = rule_part.split(sign).collect::<Vec<&str>>()[0];
                        let val_to_check = rule_part.split(sign).collect::<Vec<&str>>()[1].split(':').collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
                        let answer = rule_part.split(sign).collect::<Vec<&str>>()[1].split(':').collect::<Vec<&str>>()[1];
                        
                        if (workflow.get(var_to_check).unwrap() > &val_to_check && sign == '>') ||
                            (workflow.get(var_to_check).unwrap() < &val_to_check && sign == '<') {
                            current_rule = answer.to_string();
                            answer_found = true;
                            break;
                        }
                    }
                }
                // Don't go to next rule if we found the answer
                if answer_found {
                    break;
                }
            }
            if !answer_found {
                current_rule = rules_to_check.last().unwrap().to_string();
            }
        }

        // Count if accepted
        if current_rule == "A" {
            println!("Accepted: {:?}", workflow);
            let workflow_score = workflow.iter().map(|(_, v)| v).sum::<u32>();
            accepted_score += workflow_score;
        }
    }

    println!("Accepted score: {}", accepted_score);
}
